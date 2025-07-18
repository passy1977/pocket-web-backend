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

#include "pocket-bridge/group.h"

#include "pocket-pods/group.hpp"
using pocket::pods::group;

#include <cstring>
#include <new>

using namespace std;

pocket_group_t* pocket_group_new()
{
    auto group = new(nothrow) pocket_group_t {
        .id = 0,
        .server_id = 0,
        .user_id = 0,
        .group_id = 0,
        .server_group_id = 0,
        .title = nullptr,
        .icon = nullptr,
        .note = nullptr,
        .synchronized = false,
        .deleted = false,
        .timestamp_creation = 0,
        .has_child = false
    };
    if (!group) return nullptr;


    return group;
}

pocket_group_t* pocket_group_new_with_params(int64_t id,
                                          int64_t server_id,
                                          int64_t user_id,
                                          int64_t group_id,
                                          int64_t server_group_id,
                                          const char *title,
                                          const char *icon,
                                          const char *note,
                                          bool synchronized,
                                          bool deleted,
                                          uint64_t timestamp_creation,
                                          bool has_child)
{
    auto group = new(nothrow) pocket_group_t;
    if (!group) return nullptr;

    group->id = id;
    group->server_id = server_id;
    group->user_id = user_id;
    group->group_id = group_id;
    group->server_group_id = server_group_id;

    group->title = strdup(title);
    group->icon = strdup(icon);
    group->note = strdup(note);

    if (!group->title || !group->icon || !group->note)
    {
        pocket_group_free(group);
        return nullptr;
    }

    group->synchronized = synchronized;
    group->deleted = deleted;
    group->timestamp_creation = timestamp_creation;
    group->has_child = has_child;
    return group;
}

void pocket_group_free(pocket_group_t *group)
{
    if (group)
    {
        if (group->title)
        {
            delete group->title;
            group->title = nullptr;
        }

        if (group->icon)
        {
            delete group->icon;
            group->icon = nullptr;
        }

        if (group->note)
        {
            delete group->note;
            group->note = nullptr;
        }

        delete group;
    }
}

pocket_group_t* convert(const group::ptr& group)
{                              
    return ::pocket_group_new_with_params(
        group->id,
        group->server_id,
        group->user_id,
        group->group_id,
        group->server_group_id,
        group->title.c_str(),
        group->icon.c_str(),
        group->note.c_str(),
        group->synchronized,
        group->deleted,
        group->timestamp_creation,
        false
    );
}