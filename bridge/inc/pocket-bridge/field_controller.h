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

#ifndef POCKET_BRIDGE_FIELD_CONTROLLER_H
#define POCKET_BRIDGE_FIELD_CONTROLLER_H

#include "pocket-bridge/constants.h"
#include "pocket-bridge/field.h"
#include "pocket-bridge/pocket.h"

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct pocket_field_controller_t
{
    pocket_t* pocket;
    bool reachability;
    void* view_field;
} pocket_field_controller_t;

struct pocket_group_t;
typedef struct pocket_group_t pocket_group_t;


pocket_field_controller_t* pocket_field_controller_new(pocket_t* pocket);
void pocket_field_controller_free(const pocket_field_controller_t* self);

void pocket_field_controller_init(pocket_field_controller_t* self);
pocket_field_t** pocket_field_controller_get_list(const pocket_field_controller_t* self, int64_t group_id, const char *search, int *count);
void pocket_field_controller_free_list(pocket_field_t** list, int count);
pocket_stat_t pocket_field_controller_persist(const pocket_field_controller_t* self, const pocket_field_t* f);
pocket_stat_t pocket_field_controller_del(const pocket_field_controller_t* self, pocket_field_t* f);
int32_t pocket_field_controller_size(const pocket_field_controller_t* self, int64_t group_id);
pocket_field_t* pocket_field_controller_get(const pocket_field_controller_t* self, int64_t group_id);
int32_t pocket_field_controller_count_child(const pocket_field_controller_t* self, const pocket_group_t* field);

#ifdef __cplusplus
}
#endif

#endif //POCKET_BRIDGE_FIELD_CONTROLLER_H
