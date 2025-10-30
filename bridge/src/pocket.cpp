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

#include "pocket-services/network.hpp"
using services::network;

#include "pocket-bridge/pocket.h"
#include "pocket-bridge/user.h"
#include "pocket-bridge/constants.h"

#include <memory>
#include <new>
#include <cstring>
using namespace std;

namespace
{
    constexpr char APP_TAG[] = "POCKET";
}

extern pocket_user_t* convert(const user::opt_ptr& user);
extern user::opt_ptr convert(const pocket_user_t* pocket_user);

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

void pocket_free(pocket_t* pocket)
{
    if (pocket == nullptr) return;

    if (pocket->session)
    {
        delete static_cast<session *>(pocket->session);
        pocket->session = nullptr;
    }

    if (pocket->user)
    {
        delete static_cast<pocket_user_t*>(pocket->user);
        pocket->user = nullptr;
    }

    if (pocket->aes)
    {
        delete static_cast<aes *>(pocket->aes);
        pocket->aes = nullptr;
    }


    delete pocket;
    pocket = nullptr;
}

bool pocket_initialize_aes(pocket_t* self, const char* passwd)
{
    if (self == nullptr || passwd == nullptr)
    {
        return false;
    }

    if(self->session && self->aes)
    {
        return true;
    }

    self->aes = new(nothrow) aes(DEVICE_AES_CBC_IV, passwd);
    if(self->aes == nullptr)
    {
        error(APP_TAG, "Impossible alloc aes");
        return false;
    }
    return true;
}

bool pocket_initialize(pocket_t* self, const char* base_path, const char* config_json, bool encrypted, const char* passwd) try
{
    if (self == nullptr || base_path == nullptr || config_json == nullptr || passwd == nullptr)
    {
        return false;
    }

    if(self->aes == nullptr)
    {
        if(!pocket_initialize_aes(self, passwd))
        {
            return false;
        }
    }
    auto aes = static_cast<class aes*>(self->aes);

    if(self->session == nullptr)
    {
        auto session = new(nothrow) class session(encrypted ? aes->decrypt(config_json) : config_json, base_path);
        if(session == nullptr)
        {
            if(aes)
            {
                delete aes;
                aes = nullptr;
            }
            error(APP_TAG, "Impossible alloc session");
            return false;
        }

        session->init();
        self->session = session;
    }

    return true;
}
catch (const runtime_error& e)
{
    auto session = static_cast<class session*>(self->session);
    auto aes = static_cast<class aes*>(self->aes);
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


pocket_stat_t pocket_login(pocket_t* self, const char* email, const char* passwd) try
{
    if(self == nullptr && email == nullptr || passwd == nullptr)
    {
        return ERROR;
    }

    auto session = static_cast<class session*>(self->session);

#ifdef SYNCHRONIZER_TIMEOUT
    session->set_synchronizer_timeout(SYNCHRONIZER_TIMEOUT);
#endif

#ifdef SYNCHRONIZER_CONNECT_TIMEOUT
    session->set_synchronizer_connect_timeout(SYNCHRONIZER_CONNECT_TIMEOUT);
#endif
    if(auto&& user = session->login(email, passwd, POCKET_ENABLE_AES); user.has_value())
    {
        session->send_data(user);

        if (self->user)
        {
            pocket_user_free(static_cast<pocket_user_t*>(self->user));
            self->user = nullptr;
        }
        self->user = convert(user);
        return OK;
    }
    else
    {
        return static_cast<pocket_stat_t>(session->get_status());
    }
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return static_cast<pocket_stat_t>(static_cast<session*>(self->session)->get_status());
}

pocket_stat_t pocket_logout(const pocket_t *self, bool maintain_config) try
{
    auto session = static_cast<class session*>(self->session);
    const auto user = convert(static_cast<pocket_user_t*>(self->user));

#ifdef SYNCHRONIZER_TIMEOUT
    session->set_synchronizer_timeout(SYNCHRONIZER_TIMEOUT);
#endif

#ifdef SYNCHRONIZER_CONNECT_TIMEOUT
    session->set_synchronizer_connect_timeout(SYNCHRONIZER_CONNECT_TIMEOUT);
#endif
    if(maintain_config) 
    {
        return session->invalidate(user) ? OK : static_cast<pocket_stat_t>(session->get_status());
    } 
    else
    {
        return session->logout(user) ? OK : static_cast<pocket_stat_t>(session->get_status());
    }
    
}
catch(const runtime_error& e)
{
    auto session = static_cast<class session*>(self->session);
    error(APP_TAG, e.what());
    return static_cast<pocket_stat_t>(session->get_status());
}

pocket_stat_t pocket_change_passwd(pocket_t* self, const char* full_path_file, const char* config_json, const char* new_passwd) try
{
    if(self == nullptr || new_passwd == nullptr || strlen(new_passwd) == 0)
    {
        return ERROR;
    }

    auto session = static_cast<class session*>(self->session);


    session->set_synchronizer_timeout(0);
    session->set_synchronizer_connect_timeout(0);
    if( auto&& user_opt = session->change_passwd(convert(static_cast<pocket_user_t*>(self->user)), full_path_file, new_passwd, POCKET_ENABLE_AES); user_opt.has_value())
    {
//        self->user = convert(user_opt);
//
//        if(self->aes)
//        {
//            delete static_cast<class aes*>(self->aes);
//            self->aes = nullptr;
//        }
//        self->aes = new(nothrow) class aes(DEVICE_AES_CBC_IV, new_passwd);
//        if(self->aes == nullptr)
//        {
//            if(self->session)
//            {
//                delete static_cast<class session*>(self->session);
//                self->session = nullptr;
//            }
//            error(APP_TAG, "Impossible alloc aes");
//            return ERROR;
//        }

        return OK;
    }
    else
    {
        return static_cast<pocket_stat_t>(session->get_status());
    }
}
catch(const runtime_error& e)
{
    auto session = static_cast<class session*>(self->session);
    error(APP_TAG, e.what());
    return static_cast<pocket_stat_t>(session->get_status());
}

bool pocket_copy_group(pocket_t* self, int64_t group_id_src, int64_t group_id_dst, bool move) try
{
    if(self == nullptr)
    {
        return ERROR;
    }

    auto session = static_cast<class session*>(self->session);
    const auto user = convert(static_cast<pocket_user_t*>(self->user));

    bool ret = session->copy_group(user, static_cast<int64_t>(group_id_src), static_cast<int64_t>(group_id_dst), move);
    if(ret)
    {
        if(auto u = session->send_data(user); u.has_value())
        {
            if (self->user)
            {
                pocket_user_free(static_cast<pocket_user_t*>(self->user));
                self->user = nullptr;
            }
            self->user = convert(user);
        }
        else
        {
            ret = false;
        }
    }
    
    return ret;
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return false;
}


bool pocket_copy_field(pocket_t* self, int64_t field_id_src, int64_t group_id_dst, bool move) try
{
        if(self == nullptr)
    {
        return ERROR;
    }

    auto session = static_cast<class session*>(self->session);
    const auto user = convert(static_cast<pocket_user_t*>(self->user));

    bool ret = session->copy_field(user, static_cast<int64_t>(field_id_src), static_cast<int64_t>(group_id_dst), move);
    if(ret)
    {
        if(auto u = session->send_data(user); u.has_value())
        {
            if (self->user)
            {
                pocket_user_free(static_cast<pocket_user_t*>(self->user));
                self->user = nullptr;
            }
            self->user = convert(user);
        }
        else
        {
            ret = false;
        }
    }
    
    return ret;

}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return false;
}

pocket_stat_t pocket_send_data_with_timeouts(pocket_t* self, long timeout, long connect_timeout) try
{
    if (!self || self->user == nullptr) return ERROR;

    auto session = static_cast<class session*>(self->session);
    const auto logged_user = convert(static_cast<pocket_user_t*>(self->user));

    session->set_synchronizer_timeout(timeout);
    session->set_synchronizer_connect_timeout(connect_timeout);
    if(auto&& user = session->send_data(logged_user); user.has_value())
    {
        pocket_user_free(static_cast<pocket_user_t*>(self->user));
        self->user = convert(user);
    }

    return static_cast<pocket_stat_t>(session->get_status());
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return ERROR;
}

pocket_stat_t pocket_send_data(pocket_t* self) try 
{
#if defined(SYNCHRONIZER_TIMEOUT) && defined(SYNCHRONIZER_CONNECT_TIMEOUT) 
    return pocket_send_data_with_timeouts(self, SYNCHRONIZER_TIMEOUT, SYNCHRONIZER_CONNECT_TIMEOUT);
#else
    return pocket_send_data_with_timeouts(self, 0, 0);
#endif
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return ERROR;
}


bool pocket_group_controller_data_export(const pocket_t* self, const char* full_path_file_export) try
{
    if (!full_path_file_export) return false;

    if (!self || self->user == nullptr) return false;

    auto session = static_cast<class session*>(self->session);
    const auto user = convert(static_cast<pocket_user_t*>(self->user));

#ifdef SYNCHRONIZER_TIMEOUT
    session->set_synchronizer_timeout(SYNCHRONIZER_TIMEOUT);
#endif

#ifdef SYNCHRONIZER_CONNECT_TIMEOUT
    session->set_synchronizer_connect_timeout(SYNCHRONIZER_CONNECT_TIMEOUT);
#endif

    return session->export_data(user, full_path_file_export, POCKET_ENABLE_AES);
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return false;
}

bool pocket_group_controller_data_import(pocket_t* self, const char* full_path_file_import) try
{
    if (!full_path_file_import) return false;


    if (!self || self->user == nullptr) return false;

    auto session = static_cast<class session*>(self->session);
    const auto user = convert(static_cast<pocket_user_t*>(self->user));

    if (session->import_data(user, full_path_file_import, POCKET_ENABLE_AES)) 
    {
        auto rc = pocket_send_data_with_timeouts(self, 0, 0);
        if(rc == OK || rc == READY) 
        {
            return true;
        }
        else
        {
            return false;
        }
    }
    else
    {
        return false;
    }
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return false;
}

bool pocket_group_controller_data_import_legacy(pocket_t* self, const char* full_path_file_import) try
{
    if (!full_path_file_import) return false;

    if (!self || self->user == nullptr) return false;

    auto session = static_cast<class session*>(self->session);
    const auto user = convert(static_cast<pocket_user_t*>(self->user));

    if (session->import_data_legacy(user, full_path_file_import, POCKET_ENABLE_AES)) 
    {
        auto rc = pocket_send_data_with_timeouts(self, 0, 0);
        if(rc == OK || rc == READY) 
        {
            return true;
        }
        else
        {
            return false;
        }
    }
    else
    {
        return false;
    }
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return false;
}

bool pocket_is_no_network(const pocket_t* self)
{
    if (self == nullptr || self->session == nullptr || self->user == nullptr)
    {
        return false;
    }
    return static_cast<class session*>(self->session)->is_no_network();
}

bool pocket_heartbeat(const pocket_t* self)
{
    if (self == nullptr || self->session == nullptr || self->user == nullptr)
    {
        return false;
    }
    auto session = static_cast<class session*>(self->session);

    if(session->is_no_network())
    {
        return false;
    }

    const auto user = convert(static_cast<pocket_user_t*>(self->user));
    if(!user)
    {
        return false;
    }

    return session->heartbeat(user);
}

const char* pocket_aes_decrypt(const pocket_t* self, const char encrypted[])
{
    if (self == nullptr || encrypted == nullptr || self->aes == nullptr)
    {
        return nullptr;
    }
    auto aes = static_cast<class aes*>(self->aes);

    auto dec = aes->decrypt(encrypted);

    auto ret = static_cast<char*>(malloc(dec.size() + 1));
    if (ret == nullptr)
    {
        return nullptr;
    }
    memcpy(ret, dec.c_str(), dec.length());

    return ret;
}

const char* pocket_aes_encrypt(const pocket_t* self, const char plain[])
{
    if (self == nullptr || plain == nullptr || self->aes == nullptr)
    {
        return nullptr;
    }
    auto aes = static_cast<class aes*>(self->aes);

    auto enc = aes->encrypt(plain);
    auto ret = static_cast<char*>(malloc(enc.length() + 1));
    if (ret == nullptr)
    {
        return nullptr;
    }
    memset(ret, '\0', enc.length() + 1);
    memcpy(ret, enc.c_str(), enc.length());

    return ret;
}

const char* pocket_sha512_encrypt(const char str[])
{
    auto enc = crypto_encode_sha512(str);
    auto ret = static_cast<char*>(malloc(enc.length() + 1));
    if (ret == nullptr)
    {
        return nullptr;
    }
    memset(ret, '\0', enc.length() + 1);
    memcpy(ret, enc.c_str(), enc.length());
    return ret;
}

char *pocket_strdup(const char *str)
{
     if (str == nullptr) return nullptr;
     size_t len = strlen(str);
     char* ret = new(nothrow) char[len + 1];
     if (ret == nullptr) return nullptr;
     memcpy(ret, str, len);
     ret[len] = '\0';
     return ret;
}