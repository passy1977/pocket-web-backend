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

#include "pocket/field_controller.h"

field_controller_t* pocket_field_controller_init(void) {
    return nullptr;
}

void pocket_field_controller_initialize(field_controller_t* self)
{

}

pocket_field_t** pocket_field_controller_get_list_field(field_controller_t* self, pocket_stat_t group_id, const char* search)
{

    return nullptr;
}

pocket_stat_t pocket_field_controller_persist_field(field_controller_t* self, const pocket_field_t* f)
{

    return OK;
}

pocket_stat_t pocket_field_controller_del_field(field_controller_t* self, pocket_field_t* f)
{

    return OK;
}

int32_t pocket_field_controller_size_filed(field_controller_t* self, pocket_stat_t group_id)
{

    return 0;
}

pocket_field_t* pocket_field_controller_get_filed(field_controller_t* self, pocket_stat_t group_id)
{

    return nullptr;
}