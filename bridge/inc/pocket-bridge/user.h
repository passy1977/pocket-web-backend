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


#ifndef POCKET_BRIDGE_USER_H
#define POCKET_BRIDGE_USER_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif


typedef enum {
    USER_STAT_NOT_ACTIVE = 1,
    USER_STAT_ACTIVE = 0,
    USER_STAT_DELETED = 2,
    USER_STAT_INVALIDATED = 3
} user_stat_t;

typedef struct {
    int64_t id;
    char* email;
    char* name;
    char* passwd;
    user_stat_t status;
    uint64_t timestamp_last_update;
} pocket_user_t;

pocket_user_t* pocket_user_init(void);
pocket_user_t* pocket_user_init_with_params(int64_t id,
                                            const char *email,
                                            const char *name,
                                            const char *passwd,
                                            uint64_t timestamp_last_update,
                                            user_stat_t status);

void pocket_user_free(pocket_user_t *user);

#ifdef __cplusplus
}
#endif


#endif //POCKET_BRIDGE_USER_H
