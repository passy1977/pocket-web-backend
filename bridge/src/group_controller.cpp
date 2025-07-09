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
 

pocket_group_controller_t* pocket_group_controller_init(pocket_t* pocket)
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
        .view_field = nullptr,
        .show_list = nullptr
    };
}

void pocket_group_controller_initialize(pocket_group_controller_t* controller) 
{
    if (controller && controller->pocket && controller->pocket->session)
    {
        auto session = static_cast<class session*>(controller->pocket->session);
        controller->view_group = session->get_view_group().get();
        controller->view_group_field = session->get_view_group_field().get();
        controller->view_field = session->get_view_field().get();
    }
}


pocket_group_t** pocket_get_list_group(pocket_group_controller_t *controller, uint32_t group_id, const char *search, int *count) try
{
    if (!controller || !count) return nullptr;

    auto view_group = static_cast<view<group> *>(controller->view_group);


    auto&& list = view_group->get_list(group_id, search);
    *count = list.size();

    auto ret = new(nothrow) pocket_group_t[*count];
    if (ret == nullptr)
    {
        return nullptr;
    }

    int32_t i = 0;
    for(auto &&it : list)
    {
        ret[i] = *it;
        i++;
    }
    return nullptr;
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return nullptr;    
}

int32_t pocket_count_child(const pocket_group_t* group)
{
    if (!group) return -1;

    return 0; // Placeholder
}

pocket_stat_t pocket_del_group(pocket_group_controller_t *controller, const pocket_group_t* group)
{

    return OK; // Placeholder
}

pocket_stat_t pocket_persist_group(pocket_group_controller_t *controller, const pocket_group_t* group)
{

    return OK; // Placeholder
}

pocket_group_t* pocket_get_group(pocket_group_controller_t *controller, uint32_t group_id)
{
    if (!controller) return nullptr;

    return nullptr; // Placeholder
}

uint32_t pocket_get_last_id_group_field(void)
{

    return 0; // Placeholder
}

bool pocket_data_export(const char *fullPathFileExport)
{
    if (!fullPathFileExport) return false;

    return true; // Placeholder
}

bool pocket_data_import(const char *fullPathFileImport)
{
    if (!fullPathFileImport) return false;

    // Implementare la logica di importazione dei dati
    return true; // Placeholder
}

bool pocket_data_import_legacy(const char *fullPathFileImport)
{
    if (!fullPathFileImport) return false;

    return true; // Placeholder
}

void pocket_clean_show_list(pocket_group_controller_t* controller)
{

}

void pocket_fill_show_list(pocket_group_controller_t* controller, const pocket_group_t *group, bool insert)
{

}

pocket_show_list_t* pocket_get_show_list(void)
{
    return nullptr;
}

bool pocket_add_to_show_list(pocket_group_controller_t *controller, const pocket_group_field_t *groupField)
{
    if (!controller || !groupField) return false;

    return true; // Placeholder
}

bool pocket_del_from_show_list(pocket_group_controller_t *controller, uint32_t idGroupField)
{
    if (!controller) return false;

    return true; // Placeholder
}

uint8_t pocket_size_show_list(const pocket_group_controller_t* controller)
{
    if (!controller) return 0;

    return 0; // Placeholder
}