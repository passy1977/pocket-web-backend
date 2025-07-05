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

#include "pocket-bridge/user.h"

#include <new>
#include <cstring>

using namespace std;

pocket_user_t* pocket_user_init()
{
    auto user = new(nothrow) pocket_user_t;
    if (!user) return nullptr;

    user->id = 0;
    user->email = nullptr;
    user->name = nullptr;
    user->passwd = nullptr;
    user->status = USER_STAT_NOT_ACTIVE; // Imposta lo stato predefinito
    user->timestamp_last_update = 0;

    return user;
}

pocket_user_t* pocket_user_init_with_id(uint32_t id,
                                        const char *email,
                                        const char *name,
                                        const char *passwd,
                                        uint64_t timestamp_last_update,
                                        user_stat_t status) {
    auto user = new(nothrow) pocket_user_t;
    if (!user) return nullptr;

    user->id = id;
    user->email = strdup(email);
    user->name = strdup(name);
    user->passwd = strdup(passwd);

    if (user->email == nullptr || user->name == nullptr || user->passwd == nullptr)
    {
        pocket_user_free(user);
        return nullptr;
    }

    user->status = status;
    user->timestamp_last_update = timestamp_last_update;
    return user;
}

void pocket_user_free(pocket_user_t *user)
{
    if (user) {
        if (user->email)
        {
            delete user->email;
            user->email = nullptr;
        }

        if (user->name)
        {
            delete user->name;
            user->name = nullptr;
        }

        if (user->passwd)
        {
            for (size_t i = 0;i < strlen(user->passwd); i++)
            {
                user->passwd[i] = '\0';
            }
            delete user->passwd;
            user->passwd = nullptr;
        }

        delete user;
    }
}