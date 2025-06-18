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


#include "pocket/pocket.h"

#include <cstring>

#include "pocket/globals.hpp"
#include <new>
using namespace std;


pocket_t* pocket_new(void)
{
    const auto pocket = new(nothrow) pocket_t;
    if (pocket == nullptr)
    {
        return nullptr;
    }

    memset(pocket, 0, sizeof(pocket_t));

    return pocket;
}

void pocket_free(const pocket_t* pocket)
{
    if (pocket == nullptr) return;
    delete pocket;
}

pocket_stat_t pocket_initialize(pocket_t* self, const char* base_path, const char* config_json,
    const char* passwd)
{
    return OK;
}

pocket_stat_t pocket_login(pocket_t* self, const char* email, const char* passwd)
{
    return OK;
}

pocket_stat_t pocket_logout(pocket_t* self, bool soft_logout)
{
    return OK;
}

pocket_stat_t pocket_change_passwd(pocket_t* self, const char* full_path_file, const char* new_passwd)
{
    return OK;
}

bool pocket_copy_group(pocket_t* self, int64_t group_id_src, int64_t group_id_dst, bool move)
{
    return true;
}

bool pocket_copy_field(pocket_t* self, int64_t field_id_src, int64_t group_id_dst, bool move)
{
    return OK;
}

pocket_stat_t pocket_send_data(pocket_t* self)
{
    return OK;
}

pocket_session_t* pocket_get_session(const pocket_t* self)
{
    return nullptr;
}