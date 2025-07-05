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


#ifndef POCKET_BRIDGE_GROUP_CONTROLLER_H
#define POCKET_BRIDGE_GROUP_CONTROLLER_H

#include "pocket-bridge/constants.h"
#include "pocket-bridge/group.h"
#include "pocket-bridge/group_field.h"
#include "pocket-bridge/field.h"


#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif


typedef struct {
    bool reachability;
} pocket_group_controller_t;

typedef struct {
    pocket_group_field_t **fields;
    size_t count;
} pocket_show_list_t;

pocket_group_controller_t* pocket_group_controller_init() ;

void pocket_group_controller_initialize(pocket_group_controller_t* controller);
pocket_group_t** pocket_get_list_group(pocket_group_controller_t *controller, uint32_t groupId, const char *search, int *count);
int32_t pocket_count_child(const pocket_group_t* group);
pocket_stat_t pocket_del_group(pocket_group_controller_t *controller, const pocket_group_t* group);
pocket_stat_t pocket_persist_group(pocket_group_controller_t *controller, const pocket_group_t* group);
pocket_group_t* pocket_get_group(pocket_group_controller_t *controller, uint32_t groupId);
uint32_t pocket_get_last_id_group_field(void);
bool pocket_data_export(const char *fullPathFileExport);
bool pocket_data_import(const char *fullPathFileImport);
bool pocket_data_import_legacy(const char *fullPathFileImport);
void pocket_clean_show_list(pocket_group_controller_t* controller);
void pocket_fill_show_list(pocket_group_controller_t* controller, const pocket_group_t *group, bool insert);
pocket_show_list_t* pocket_get_show_list(void);
bool pocket_add_to_show_list(pocket_group_controller_t *controller, const pocket_group_field_t *groupField);
bool pocket_del_from_show_list(pocket_group_controller_t *controller, uint32_t idGroupField);
uint8_t pocket_size_show_list(const pocket_group_controller_t* controller);

#ifdef __cplusplus
}
#endif

#endif //GROUP_CONTROLLER_HPP
