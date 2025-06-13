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

#include "pocket/group_field.h"

#include <new>
#include <cstring>

using namespace std;

pocket_group_field_t* pocket_group_field_init() {
    auto group_field = new(std::nothrow) pocket_group_field_t {
        .new_insertion = false,
        .id = 0,
        .server_id = 0,
        .group_id = 0,
        .server_group_id = 0,
        .title = nullptr,
        .is_hidden = false,
        .synchronized = false,
        .deleted = false,
        .timestamp_creation = 0
    };

    if (!group_field) return nullptr;

    return group_field;
}

pocket_group_field_t* pocket_group_field_init_with_id(int64_t id,
                                                      int64_t server_id,
                                                      int64_t user_id,
                                                      int64_t group_id,
                                                      int64_t server_group_id,
                                                      const char *title,
                                                      bool is_hidden,
                                                      bool synchronized,
                                                      bool deleted,
                                                      uint64_t timestamp_creation) {
    auto group_field = new(std::nothrow) pocket_group_field_t;
    if (!group_field) return nullptr;

    group_field->new_insertion = false;
    group_field->id = id;
    group_field->user_id = user_id;
    group_field->server_id = server_id;
    group_field->group_id = group_id;
    group_field->server_group_id = server_group_id;
    
    group_field->title = strdup(title);
    if (!group_field->title)
    {
        delete group_field;
        return nullptr;
    }

    group_field->is_hidden = is_hidden;
    group_field->synchronized = synchronized;
    group_field->deleted = deleted;
    group_field->timestamp_creation = timestamp_creation;

    return group_field;
}

void pocket_group_field_free(pocket_group_field_t *group_field)
{
    if (group_field)
    {
        if (group_field->title)
        {
            delete group_field->title;
            group_field->title = nullptr;
        }

        delete group_field;
    }
}