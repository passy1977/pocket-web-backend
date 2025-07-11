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

#include "pocket-bridge/field_controller.h"

#include "pocket-controllers/session.hpp"
using pocket::controllers::session;

#include <new>
using namespace std;

pocket_field_controller_t* pocket_field_controller_new(pocket_t* pocket) {
    if (pocket == nullptr)
    {
        return nullptr;
    }
    return new(nothrow) pocket_field_controller_t {
        .pocket = pocket,
        .reachability = true,
        .view_field = nullptr
    };
}

void pocket_field_controller_free(pocket_field_controller_t* field_controller)
{
    if (field_controller == nullptr)
    {
        return;
    }

    delete field_controller;
    field_controller = nullptr;
}

void pocket_field_controller_init(pocket_field_controller_t* self)
{
    if (self && self->pocket && self->pocket->session)
    {
        auto session = static_cast<class session*>(self->pocket->session);
        self->view_field = session->get_view_field().get();
    }
}

pocket_field_t** pocket_field_controller_get_list_field(pocket_field_controller_t* self, pocket_stat_t group_id, const char* search)
{

    return nullptr;
}

pocket_stat_t pocket_field_controller_persist_field(pocket_field_controller_t* self, const pocket_field_t* f)
{

    return OK;
}

pocket_stat_t pocket_field_controller_del_field(pocket_field_controller_t* self, pocket_field_t* f)
{

    return OK;
}

int32_t pocket_field_controller_size_filed(pocket_field_controller_t* self, pocket_stat_t group_id)
{

    return 0;
}

pocket_field_t* pocket_field_controller_get_filed(pocket_field_controller_t* self, pocket_stat_t group_id)
{

    return nullptr;
}