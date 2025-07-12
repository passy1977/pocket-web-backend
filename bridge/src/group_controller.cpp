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

#include "pocket/globals.hpp"
using namespace pocket;

#include "pocket-controllers/session.hpp"
using pocket::controllers::session;


#include "pocket-views/view-group.hpp"
#include "pocket-views/view-group-field.hpp"
using views::view;

#include "pocket-pods/group.hpp"
#include "pocket-pods/group-field.hpp"
#include "pocket-pods/field.hpp"
using namespace pods;

#include <new>
using namespace std;

namespace
{

constexpr char APP_TAG[] = "GroupController";

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
    self = NULL;
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


pocket_group_t** pocket_group_controller_get_list_group(const pocket_group_controller_t* self, uint32_t group_id, const char *search, int *count) try
{
    if (!self || !count) return nullptr;

    auto view_group = static_cast<view<group> *>(self->view_group);


    auto&& list = view_group->get_list(group_id, search);
    *count = list.size();

    auto ret = new(nothrow) pocket_group_t*[*count];
    if (ret == nullptr)
    {
        return nullptr;
    }

    int32_t i = 0;
    for(auto &&it : list)
    {
        ret[i]  = convert(it);
        i++;
    }
    return ret;
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return nullptr;    
}

int32_t pocket_group_controller_count_child(const pocket_group_controller_t* self, const pocket_group_t* group)
{
    if (self == nullptr || group == nullptr) return -1;

    return 0; // Placeholder
}

pocket_stat_t pocket_group_controller_del_group(pocket_group_controller_t* self, const pocket_group_t* group)
{

    return OK; // Placeholder
}

pocket_stat_t pocket_group_controller_persist_group(pocket_group_controller_t* self, const pocket_group_t* group)
{

    return OK; // Placeholder
}

pocket_group_t* pocket_group_controller_get_group(pocket_group_controller_t* self, uint32_t group_id)
{
    if (!self) return nullptr;

    return nullptr; // Placeholder
}

uint32_t pocket_group_controller_get_last_id_group_field(void)
{

    return 0; // Placeholder
}

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

bool pocket_group_controller_del_from_show_list(pocket_group_controller_t* self, uint32_t id_group_field)
{
    if (!self) return false;

    return true; // Placeholder
}

uint8_t pocket_group_controller_size_show_list(const pocket_group_controller_t* self)
{
    if (!self) return 0;

    return 0; // Placeholder
}