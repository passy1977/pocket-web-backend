/***************************************************************************
*
 * Pocket
 * Copyright (C) 2018/2025 Antonio Salsi <passy.linux@zresa.it>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 ***************************************************************************/

#include "pocket-bridge/group_controller.h"
#include "pocket-bridge/field_controller.h"

#include "pocket/globals.hpp"
using namespace pocket;

#include "pocket-controllers/session.hpp"
using pocket::controllers::session;


#include "pocket-views/view-group.hpp"
#include "pocket-views/view-group-field.hpp"
using views::view;

#include "pocket-pods/group.hpp"
#include "pocket-pods/group-field.hpp"
using namespace pods;

#include <new>
using namespace std;

namespace
{

constexpr char APP_TAG[] = "group_controller";

}
 
extern pocket_group_t* convert(const group::ptr& group) noexcept;

pocket_group_controller_t* pocket_group_controller_new(pocket_t* pocket)
{
    if (pocket == nullptr)
    {
        return nullptr;
    }
    return new(nothrow) pocket_group_controller_t {
        .pocket = pocket,
        .reachability = true,
        .view_group = nullptr,
        .view_group_field = nullptr,
        .show_list = nullptr
    };
}

void pocket_group_controller_free(pocket_group_controller_t* self)
{
    if (self == nullptr)
    {
        return;
    }

    delete self;
    self = nullptr;
}

void pocket_group_controller_init(pocket_group_controller_t* self) 
{
    if (self && self->pocket && self->pocket->session)
    {
        auto session = static_cast<class session*>(self->pocket->session);
        self->view_group = session->get_view_group().get();
        self->view_group_field = session->get_view_group_field().get();
    }
}


pocket_group_t** pocket_group_controller_get_list_group(const pocket_group_controller_t* self, const pocket_field_controller_t* field_controller, int64_t group_id, const char *search, int *count) try
{
    if (!self || !count) return nullptr;

    auto view_group = static_cast<view<group> *>(self->view_group);


    auto&& list = view_group->get_list(group_id, search);
    *count = static_cast<int>(list.size());

    auto ret = new(nothrow) pocket_group_t*[*count];
    if (ret == nullptr)
    {
        return nullptr;
    }

    size_t i = 0;
    for(auto &&it : list)
    {
        auto pocket_group = convert(it);

        pocket_group->has_child = pocket_group_controller_count_child(self, pocket_group) + pocket_field_controller_count_child(field_controller, pocket_group) > 0;

        ret[i]  = pocket_group;
        i++;
    }
    return ret;
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return nullptr;    
}

int32_t pocket_group_controller_count_child(const pocket_group_controller_t* self, const pocket_group_t* group) try
{
    if (self == nullptr || group == nullptr) return -1;
    auto view_group = static_cast<view<struct group> *>(self->view_group);

    return static_cast<uint32_t>(view_group->get_list(group->id).size());
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return 0;
}

pocket_stat_t pocket_group_controller_del_group(const pocket_group_controller_t* self, const pocket_field_controller_t* field_controller, const pocket_group_t* group) try
{
    if (!self || !field_controller || !group) return ERROR;

    auto session = static_cast<class session*>(self->pocket->session);
    auto logged_user = static_cast<user::opt_ptr *>(self->pocket->user);

    const auto view_group = static_cast<view<struct group> *>(self->view_group);
    const auto view_group_field = static_cast<view<group_field> *>(self->view_group_field);
    const auto view_field = static_cast<view<field> *>(field_controller->view_field);

    view_group_field->del_by_group_id(group->id);
    view_field->del_by_group_id(group->id);
    view_group->del(group->id);

    session->set_synchronizer_timeout(SYNCHRONIZER_TIMEOUT);
    session->set_synchronizer_connect_timeout(SYNCHRONIZER_CONNECT_TIMEOUT);
    if(auto&& user = session->send_data(*logged_user); user)
    {
        if (self->pocket->user)
        {
            delete[] static_cast<uint8_t *>(self->pocket->user);
        }
        self->pocket->user = new uint8_t[sizeof(user)];
        memcpy(self->pocket->user, &user, sizeof(user));
        return OK;
    }
    else
    {
        return static_cast<pocket_stat_t>(session->get_status());
    }

}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return ERROR;
}

pocket_stat_t pocket_group_controller_persist_group(const pocket_group_controller_t* self, const pocket_group_t* group) try
{
    if (!self || !group) return ERROR;

    auto session = static_cast<class session*>(self->pocket->session);
    const auto logged_user = static_cast<user::opt_ptr *>(self->pocket->user);

    const auto view_group = static_cast<view<struct group> *>(self->view_group);

    // struct group zzz {
    //     .id = group->id,
    //     .server_id = group->server_id,
    //     .group_id = group->group_id,
    //     .server_group_id = group->server_group_id,
    //     .title = group->title,
    //     .icon = group->icon,
    //     .note = group->note,
    //     .synchronized = group->synchronized,
    //     .deleted = group->deleted,
    //     .timestamp_creation = group->timestamp_creation
    // };

    auto g = make_unique<struct group>();
    g->server_id = group->server_id;
    g->group_id = group->group_id;
    g->server_group_id = group->server_group_id;
    g->title = group->title;
    g->icon = group->icon;
    g->note = group->note;
    g->deleted = group->deleted;
    g->timestamp_creation = group->timestamp_creation;
    g->user_id = logged_user->value()->id;
    g->synchronized = false;
    g->id = view_group->persist(g);

    // for (NSNumber *key in showList)
    // {
    //     GroupField *gfObjC = showList[key];
    //     auto&& gf = convert(gfObjC);
    //     gf->synchronized = false;
    //     if(gfObjC.newInsertion)
    //     {
    //         gf->id = 0;
    //         gf->user_id = user._id;
    //         gf->group_id = g->id;
    //         gf->server_group_id = g->server_id;
    //     }
    //     gf->id = viewGroupField->persist(gf);
    //     gfObjC._id = static_cast<uint32_t>(gf->id);
    //
    //     if(gfObjC.newInsertion)
    //     {
    //         Field *fObjC = [Field new];
    //         fObjC.title = gfObjC.title;
    //         fObjC.value = @"";
    //         fObjC.isHidden = gf->is_hidden;
    //         auto&& f = convert(fObjC);
    //         f->user_id = user._id;
    //         f->group_id = g->id;
    //         f->server_group_id = g->server_id;
    //         f->group_field_id = gf->id;
    //         f->server_group_id = gf->server_id;
    //         f->synchronized = false;
    //
    //         f->id = viewField->persist(f);
    //         fObjC._id = static_cast<uint32_t>(f->id);
    //     }
    // }

    session->set_synchronizer_timeout(SYNCHRONIZER_TIMEOUT);
    session->set_synchronizer_connect_timeout(SYNCHRONIZER_CONNECT_TIMEOUT);
    if(auto&& user = session->send_data(*logged_user); user)
    {
        if (self->pocket->user)
        {
            delete[] static_cast<uint8_t *>(self->pocket->user);
        }
        self->pocket->user = new uint8_t[sizeof(user)];
        memcpy(self->pocket->user, &user, sizeof(user));
    }

    return static_cast<pocket_stat_t>(session->get_status());
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return ERROR;
}

pocket_group_t* pocket_group_controller_get_group(const pocket_group_controller_t* self, int64_t group_id) try
{
    if (!self) return nullptr;

    const auto view_group = static_cast<view<struct group> *>(self->view_group);

    auto&&group_opt = view_group->get(group_id);
    if(group_opt)
    {
        return convert(*group_opt);
    }
    return nullptr;
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return nullptr;
}

// int64_t pocket_group_controller_get_last_id_group_field(void)
// {
//
//     return 0; // Placeholder
// }

bool pocket_group_controller_data_export(const char* full_path_file_export)
{
    if (!full_path_file_export) return false;

    return true; // Placeholder
}

bool pocket_group_controller_data_import(const char* full_path_file_import)
{
    if (!full_path_file_import) return false;

    // Implementare la logica di importazione dei dati
    return true; // Placeholder
}

bool pocket_group_controller_data_import_legacy(const char* full_path_file_import)
{
    if (!full_path_file_import) return false;

    return true; // Placeholder
}

void pocket_group_controller_clean_show_list(pocket_group_controller_t* controller)
{

}

void pocket_group_controller_fill_show_list(pocket_group_controller_t* controller, const pocket_group_t *group, bool insert)
{

}

pocket_show_list_t* pocket_group_controller_get_show_list(void)
{
    return nullptr;
}

bool pocket_add_to_show_list(pocket_group_controller_t* self, const pocket_group_field_t* group_field)
{
    if (!self || !group_field) return false;

    return true; // Placeholder
}

bool pocket_group_controller_del_from_show_list(pocket_group_controller_t* self, int64_t id_group_field)
{
    if (!self) return false;

    return true; // Placeholder
}

uint8_t pocket_group_controller_size_show_list(const pocket_group_controller_t* self)
{
    if (!self) return 0;

    return 0; // Placeholder
}