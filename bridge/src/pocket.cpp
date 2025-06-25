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

#include "pocket/globals.hpp"
using namespace pocket;

#include "pocket-controllers/session.hpp"
using controllers::session;

#include "pocket-services/crypto.hpp"
using namespace services;

#include "pocket-pods/user.hpp"
using pods::user;

#include "pocket/pocket.h"

#include <memory>
#include <new>
#include <cstring>
using namespace std;

namespace
{
    constexpr char APP_TAG[] = "POCKET";
}

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

bool pocket_initialize(const pocket_t* self, const char* base_path, const char* config_json, const char* fron_stored_data_config_json, const char* passwd)
{
    if(self->session && self->aes)
    {
        return true;
    }
    auto session = static_cast<class session*>(self->session);
    auto aes = static_cast<class aes*>(self->aes);

    if (fron_stored_data_config_json)
    {
        try
        {
            aes = new(nothrow) class aes(DEVICE_AES_CBC_IV, passwd);
            if(aes == nullptr)
            {
                error(APP_TAG, "Impossbile alloc aes");
                return false;
            }

            session = new(nothrow) class session(aes->decrypt(fron_stored_data_config_json), base_path);
            if(session == nullptr)
            {
                if(aes)
                {
                    delete aes;
                    aes = nullptr;
                }
                error(APP_TAG, "Impossbile alloc session");
                return false;
            }

            session->init();
            return true;
        }
        catch (const runtime_error& e)
        {
            if(session)
            {
                delete session;
                session = nullptr;
            }

            if(aes)
            {
                delete aes;
                aes = nullptr;
            }

            error(APP_TAG, e.what());
            return false;
        }
    }
    else
    {
        //no encrypt configJson and not stored

        if (config_json == nullptr)
        {
            error(APP_TAG, "configJson == nullptr");
            return false;
        }


        try
        {
            session = new(nothrow) class session(config_json, base_path);
            if(session == nullptr)
            {
                error(APP_TAG, "Impossbile alloc session");
                return false;
            }
            session->init();

            aes = new(nothrow) class aes(DEVICE_AES_CBC_IV, passwd);
            if(aes == nullptr)
            {
                if(session)
                {
                    delete session;
                    session = nullptr;
                }
                error(APP_TAG, "Impossbile alloc aes");
                return false;
            }

            [[NSUserDefaults standardUserDefaults] setObject:[NSString stringWithCString:aes->encrypt([configJson UTF8String]).c_str() encoding:NSUTF8StringEncoding] forKey: KEY_DEVICE];
            [[NSUserDefaults standardUserDefaults] synchronize];
            return true;
        }
        catch (const runtime_error& e)
        {
            if(session)
            {
                delete session;
                session = nullptr;
            }

            if(aes)
            {
                delete aes;
                aes = nullptr;
            }
            error(APP_TAG, e.what());
            return false;
        }
    }

    return true;
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
