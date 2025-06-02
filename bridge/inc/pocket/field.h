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

#ifndef POCKET_BRIDGE_FIELD_H
#define POCKET_BRIDGE_FIELD_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif




typedef struct
{
 int64_t id;
 int64_t server_id;
 int64_t user_id;
 int64_t group_id;
 int64_t server_group_id;
 int64_t group_field_id;
 int64_t server_group_field_id;
 char* title;
 char* value;
 bool is_hidden;
 bool synchronized;
 bool deleted;
 uint64_t timestamp_creation;
} pocket_field_t;

pocket_field_t pocket_field_new();
pocket_field_t pocket_field_new_with_args( int64_t id,
 int64_t server_id,
 int64_t user_id,
 int64_t group_id,
 int64_t server_group_id,
 int64_t group_field_id,
 int64_t server_group_field_id,
 const char* title,
 const char* value,
 bool is_hidden,
 bool synchronized,
 bool deleted,
 uint64_t timestamp_creation
 );

bool pocket_field_is_null(const pocket_field_t* field);

void pocket_field_free(pocket_field_t* field);

#ifdef __cplusplus
}
#endif

#endif //POCKET_BRIDGE_FIELD_H
