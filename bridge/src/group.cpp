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

#include "pocket/group.h"

#include <cstdlib>
#include <cstring>

pocket_group_t* pocket_group_init() {
    pocket_group_t *group = (pocket_group_t *)malloc(sizeof(pocket_group_t));
    if (!group) return nullptr;

    group->_id = 0;
    group->server_id = 0;
    group->user_id = 0;
    group->group_id = 0;
    group->server_group_id = 0;
    group->group_field_id = 0;
    group->server_group_field_id = 0;
    group->title = nullptr;
    group->icon = nullptr;
    group->note = nullptr;
    group->value = nullptr;
    group->is_hidden = false;
    group->synchronized = false;
    group->deleted = false;
    group->timestamp_creation = 0;

    return group;
}

pocket_group_t* pocket_group_init_with_id(uint32_t id,
                                          uint32_t server_id,
                                          uint32_t user_id,
                                          uint32_t group_id,
                                          uint32_t server_group_id,
                                          uint32_t group_field_id,
                                          uint32_t server_group_field_id,
                                          const char *title,
                                          const char *icon,
                                          const char *note,
                                          const char *value,
                                          bool is_hidden,
                                          bool synchronized,
                                          bool deleted,
                                          uint64_t timestamp_creation) {
    pocket_group_t *group = (pocket_group_t *)malloc(sizeof(pocket_group_t));
    if (!group) return nullptr;

    group->_id = id;
    group->server_id = server_id;
    group->user_id = user_id;
    group->group_id = group_id;
    group->server_group_id = server_group_id;
    group->group_field_id = group_field_id;
    group->server_group_field_id = server_group_field_id;

    // Copia sicura delle stringhe
    group->title = strdup(title);
    group->icon = strdup(icon);
    group->note = strdup(note);
    group->value = strdup(value);

    if (!group->title || !group->icon || !group->note || !group->value) {
        free(group->title);
        free(group->icon);
        free(group->note);
        free(group->value);
        free(group);
        return nullptr;
    }

    group->is_hidden = is_hidden;
    group->synchronized = synchronized;
    group->deleted = deleted;
    group->timestamp_creation = timestamp_creation;

    return group;
}

void pocket_group_free(pocket_group_t *group) {
    if (group) {
        free(group->title);
        free(group->icon);
        free(group->note);
        free(group->value);
        free(group);
    }
}