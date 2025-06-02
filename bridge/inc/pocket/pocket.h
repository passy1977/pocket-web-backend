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

#ifndef POCKET_BRIDGE_H
#define POCKET_BRIDGE_H

#include "pocket/constants.h"

#include <stdbool.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif


typedef struct
{

} pocket_session_t;

typedef struct
{

} pocket_t;

pocket_t* pocket_init(void);
pocket_t* pocket_shared(void);

pocket_stat_t pocket_initialize(pocket_t* self, const char* base_path, const char* config_json, const char* passwd);
pocket_stat_t pocket_login(pocket_t* self, const char* email, const char* passwd);
pocket_stat_t pocket_logout(pocket_t* self, bool soft_logout);
pocket_stat_t pocket_change_passwd(pocket_t* self, const char* full_path_file, const char* new_passwd);
bool pocket_copy_group(pocket_t* self, int64_t group_id_src, int64_t group_id_dst, bool move);
bool pocket_copy_field(pocket_t* self, int64_t field_id_src, int64_t group_id_dst, bool move);
pocket_stat_t pocket_send_data(pocket_t* self);

pocket_session_t* pocket_get_session(const pocket_t* self);

#ifdef __cplusplus
}
#endif

#endif //POCKET_BRIDGE_H
