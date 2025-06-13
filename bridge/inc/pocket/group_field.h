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

#ifndef POCKET_BRIDGE_GROUP_FIELD_H
#define POCKET_BRIDGE_GROUP_FIELD_H

#include <stdint.h>
#include <stdbool.h>

typedef struct {
    bool new_insertion;
    int64_t id;
    int64_t server_id;
    int64_t user_id;
    int64_t group_id;
    int64_t server_group_id;
    char* title;
    bool is_hidden;
    bool synchronized;
    bool deleted;
    uint64_t timestamp_creation;
} pocket_group_field_t;

pocket_group_field_t* pocket_group_field_init(void);
pocket_group_field_t* pocket_group_field_init_with_id(int64_t id,
                                                      int64_t server_id,
                                                      int64_t user_id,
                                                      int64_t group_id,
                                                      int64_t server_group_id,
                                                      const char *title,
                                                      bool is_hidden,
                                                      bool synchronized,
                                                      bool deleted,
                                                      uint64_t timestamp_creation);

void pocket_group_field_free(pocket_group_field_t *group_field);

#endif //GROUP_FIELD_H
