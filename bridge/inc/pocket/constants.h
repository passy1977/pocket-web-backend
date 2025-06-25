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

#ifndef POCKET_BRIDGE_CONSTANTS_H
#define POCKET_BRIDGE_CONSTANTS_H

#ifndef DEVICE_AES_CBC_IV
#define DEVICE_AES_CBC_IV "1234567890123456"
#endif


#ifdef __cplusplus
extern "C" {
#endif



typedef enum
{
    READY = 0,
    BUSY,
    USER_NOT_FOUND = 600,
    WRONG_SIZE_TOKEN = 601,
    DEVICE_ID_NOT_MATCH = 602,
    DEVICE_NOT_FOUND = 603,
    SECRET_NOT_MATCH = 604,
    PASSWD_ERROR = 605,
    TIMESTAMP_LAST_UPDATE_NOT_MATCH = 606,
    CACHE_NOT_FOND = 607,
    SECRET_EMPTY = 608,
    TIMESTAMP_LAST_NOT_PARSABLE = 609,
    ERROR = USER_NOT_FOUND + 100,
    JSON_PARSING_ERROR = USER_NOT_FOUND + 101,
    DB_GROUP_ERROR = USER_NOT_FOUND + 102,
    DB_GROUP_FIELD_ERROR = USER_NOT_FOUND + 103,
    DB_FIELD_ERROR = USER_NOT_FOUND + 104,
    DB_GENERIC_ERROR = USER_NOT_FOUND + 105,
    NO_NETWORK = USER_NOT_FOUND + 106,
    MAP_ID_ERROR = USER_NOT_FOUND + 107,
    LOCAL_DEVICE_ID_NOT_MATCH = DEVICE_ID_NOT_MATCH + 200,
    OK = 200
} pocket_stat_t;

#ifdef __cplusplus
}
#endif



#endif //CONSTANTS_H
