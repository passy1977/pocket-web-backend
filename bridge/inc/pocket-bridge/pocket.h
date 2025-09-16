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

#include "pocket-bridge/constants.h"

#include <stdbool.h>
#include <stdlib.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct
{
    void* session;
    void* user;
    void* aes;
} pocket_t;

pocket_t* pocket_new(void);
void pocket_free(pocket_t* pocket);

bool pocket_initialize_aes(pocket_t* self, const char* passwd);
bool pocket_initialize(pocket_t* self, const char* base_path, const char* config_json, bool encrypted, const char* passwd);
pocket_stat_t pocket_login(pocket_t* self, const char* email, const char* passwd);
pocket_stat_t pocket_logout(const pocket_t *self);
pocket_stat_t pocket_change_passwd(pocket_t* self, const char* full_path_file, const char* config_json, const char* new_passwd);
bool pocket_copy_group(pocket_t* self, int64_t group_id_src, int64_t group_id_dst, bool move);
bool pocket_copy_field(pocket_t* self, int64_t field_id_src, int64_t group_id_dst, bool move);
pocket_stat_t pocket_send_data_with_timeouts(pocket_t* self, long timeout, long connect_timeout);
pocket_stat_t pocket_send_data(pocket_t* self);
bool pocket_group_controller_data_export(const pocket_t* self, const char* full_path_file_export);
bool pocket_group_controller_data_import(pocket_t* self, const char* full_path_file_import);
bool pocket_group_controller_data_import_legacy(pocket_t* self, const char* full_path_file_import);


const char* pocket_aes_decrypt(const pocket_t* self, const char encrypted[]);
const char* pocket_aes_encrypt(const pocket_t* self, const char plain[]);
const char* pocket_sha512_encrypt(const char str[]);

#ifdef __cplusplus
}
#endif

#endif //POCKET_BRIDGE_H
