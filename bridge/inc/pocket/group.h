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

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t _id;
    uint32_t server_id;
    uint32_t user_id;
    uint32_t group_id;
    uint32_t server_group_id;
    uint32_t group_field_id;
    uint32_t server_group_field_id;
    char* title; // Usare char* o un altro tipo stringa C appropriato
    char* icon;
    char* note;
    char* value;
    bool is_hidden;
    bool synchronized;
    bool deleted;
    uint64_t timestamp_creation;
} pocket_group_t;

pocket_group_t* pocket_group_init(void);

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
                                          uint64_t timestamp_creation);

void pocket_group_free(pocket_group_t *group);

#ifdef __cplusplus
}
#endif

#endif //POCKET_BRIDGE_GROUP_CONTROLLER_H
