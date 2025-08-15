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

#ifndef GROUP_FIELD_CONTROLLER_H
#define GROUP_FIELD_CONTROLLER_H

#include "group_field.h"
#include "pocket-bridge/pocket.h"

#ifdef __cplusplus
extern "C" {
#endif


typedef struct pocket_group_field_controller_t {
    pocket_t* pocket;
    bool reachability;
    void* view_group_field;
} pocket_group_field_controller_t;

pocket_group_field_controller_t* pocket_group_field_controller_new(pocket_t* pocket);
void pocket_group_field_controller_free(const pocket_group_field_controller_t* self);

void pocket_group_field_controller_init(pocket_group_field_controller_t* self);
pocket_group_field_t** pocket_group_field_controller_get_list(const pocket_group_field_controller_t* self, int64_t group_id, const char *search, int *count);
void pocket_group_field_controller_free_list(pocket_group_field_t** list, int count);

pocket_stat_t pocket_group_field_controller_del(const pocket_group_field_controller_t* self, const pocket_group_field_t* group_field);
pocket_stat_t pocket_group_field_controller_persist(const pocket_group_field_controller_t* self, const pocket_group_field_t* group_field);

#ifdef __cplusplus
}
#endif


#endif //GROUP_FIELD_CONTROLLER_H
