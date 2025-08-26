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
#include "pocket-bridge/group_field_controller.h"
#include "pocket-bridge/field_controller.h"

#include "pocket/globals.hpp"
using namespace pocket;

#include "pocket-controllers/session.hpp"
using controllers::session;

#include "pocket-views/view-group.hpp"
using views::view;

#include "pocket-pods/group.hpp"
#include "pocket-pods/group-field.hpp"
using namespace pods;

#include "pocket-bridge/user.h"

#include <new>
#include <ranges>
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
    };
}

void pocket_group_controller_free(const pocket_group_controller_t* self)
{
    if (self == nullptr)
    {
        return;
    }

    delete self;
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


pocket_group_t** pocket_group_controller_get_list(const pocket_group_controller_t* self, const pocket_field_controller_t* field_controller, int64_t group_id, const char *search, int *count) try
{
    if (!self || !count) return nullptr;

    auto view_group = static_cast<view<group> *>(self->view_group);


    auto&& list = view_group->get_list(group_id, search);
    *count = static_cast<int>(list.size());

    const auto ret = new(nothrow) pocket_group_t*[*count];
    if (ret == nullptr)
    {
        return nullptr;
    }

    size_t i = 0;
    for(auto &&it : list)
    {
        const auto pocket_group = convert(it);

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

void pocket_group_controller_free_list(pocket_group_t** list, int count)
{
    if (list == nullptr)
    {
        return;
    }

    for (int i : std::views::iota(0, count)) {
        pocket_group_free(list[i]);
    }

    delete [] list;
}

int32_t pocket_group_controller_count_child(const pocket_group_controller_t* self, const pocket_group_t* group) try
{
    if (self == nullptr || group == nullptr) return -1;
    const auto view_group = static_cast<view<struct group> *>(self->view_group);

    return static_cast<uint32_t>(view_group->get_list(group->id).size());
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return 0;
}

pocket_stat_t pocket_group_controller_del(const pocket_group_controller_t* self, const pocket_group_field_controller_t* group_field_controller, const pocket_field_controller_t* field_controller, const pocket_group_t* group) try
{
    if (!self || !group_field_controller || !field_controller || !group) return ERROR;

    auto session = static_cast<class session*>(self->pocket->session);

    const auto view_group = static_cast<view<struct group> *>(self->view_group);
    const auto view_group_field = static_cast<view<group_field> *>(group_field_controller->view_group_field);
    const auto view_field = static_cast<view<field> *>(field_controller->view_field);

    view_group_field->del_by_group_id(group->id);
    view_field->del_by_group_id(group->id);
    view_group->del(group->id);

    return READY;
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return ERROR;
}

pocket_stat_t pocket_group_controller_persist(const pocket_group_controller_t* self, pocket_group_t* group) try
{
    if (!self || !group) return ERROR;

    auto session = static_cast<class session*>(self->pocket->session);
    const auto logged_user = static_cast<pocket_user_t*>(self->pocket->user);

    const auto view_group = static_cast<view<struct group> *>(self->view_group);

    auto g = make_unique<struct group>();
    if (group->id <= 0)
    {
        g->id = 0;
    }
    else
    {
        g->id = group->id;
    }
    g->server_id = group->server_id;
    g->group_id = group->group_id;
    g->server_group_id = group->server_group_id;
    g->title = group->title;
	if(group->icon)
	{
    	g->icon = group->icon;
	}
	else
	{
		g->icon = "";
	}
	if(group->note)
	{
		g->note = group->note;
	}
	else
	{
		g->note = "";
	}
    g->deleted = group->deleted;
    g->timestamp_creation = group->timestamp_creation;
    g->user_id = logged_user->id;
    g->synchronized = false;
    auto id = view_group->persist(g);

    if (group->id <= 0)
    {
        group->id = id;
    }
    else
    {
        group->id = g->id;
    }

    return READY;
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return ERROR;
}

pocket_group_t* pocket_group_controller_get(const pocket_group_controller_t* self, int64_t group_id) try
{
    if (!self) return nullptr;

    const auto view_group = static_cast<view<struct group> *>(self->view_group);

    if(auto&&group_opt = view_group->get(group_id))
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


bool pocket_group_controller_data_export(const char* full_path_file_export)
{
    if (!full_path_file_export) return false;

    //TODO: not impl
    return true; // Placeholder
}

bool pocket_group_controller_data_import(const char* full_path_file_import)
{
    if (!full_path_file_import) return false;

    //TODO: not impl
    return true; // Placeholder
}

bool pocket_group_controller_data_import_legacy(const char* full_path_file_import)
{
    if (!full_path_file_import) return false;

    //TODO: not impl
    return true; // Placeholder
}
