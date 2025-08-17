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

#include "pocket-bridge/field_controller.h"

#include "pocket-bridge/group.h"
#include "pocket-bridge/user.h"
#include "pocket/globals.hpp"
using namespace pocket;

#include "pocket-controllers/session.hpp"
using controllers::session;


#include "pocket-views/view.hpp"
using views::view;

#include "pocket-pods/field.hpp"
using namespace pods;

#include <new>
#include <ranges>
using namespace std;

namespace
{

constexpr char APP_TAG[] = "field_controller";

}

extern pocket_field_t* convert(const field::ptr& field);

pocket_field_controller_t* pocket_field_controller_new(pocket_t* pocket) {
    if (pocket == nullptr)
    {
        return nullptr;
    }
    return new(nothrow) pocket_field_controller_t {
        .pocket = pocket,
        .reachability = true,
        .view_field = nullptr
    };
}

void pocket_field_controller_free(const pocket_field_controller_t* field_controller)
{
    if (field_controller == nullptr)
    {
        return;
    }

    delete field_controller;
    field_controller = nullptr;
}

void pocket_field_controller_init(pocket_field_controller_t* self)
{
    if (self && self->pocket && self->pocket->session)
    {
        auto session = static_cast<class session*>(self->pocket->session);
        self->view_field = session->get_view_field().get();
    }
}

pocket_field_t** pocket_field_controller_get_list(const pocket_field_controller_t* self, int64_t group_id, const char* search, int *count) try
{
    if (!self || !count) return nullptr;

    auto view_field = static_cast<view<field> *>(self->view_field);


    auto&& list = view_field->get_list(group_id, search);
    *count = list.size();

    const auto ret = new(nothrow) pocket_field_t*[*count];
    if (ret == nullptr)
    {
        return nullptr;
    }

    size_t i = 0;
    for(auto &&it : list)
    {
        ret[i] = convert(it);
        i++;
    }
    return ret;
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return nullptr;
}

void pocket_field_controller_free_list(pocket_field_t** list, int count)
{
    if (list == nullptr)
    {
        return;
    }

    for (int i : std::views::iota(0, count)) {
        pocket_field_free(list[i]);
    }

    delete [] list;
}


pocket_stat_t pocket_field_controller_persist(const pocket_field_controller_t* self, pocket_field_t* field) try
{
    if (!self || !field) return ERROR;

    auto session = static_cast<class session*>(self->pocket->session);
    const auto logged_user = static_cast<pocket_user_t*>(self->pocket->user);

    const auto view_field = static_cast<view<struct group_field> *>(self->view_field);

    auto f = make_unique<struct group_field>();
    if (field->id < 0)
    {
        f->id = 0;
    }
    else
    {
        f->id = field->id;
    }
    f->id = field->id;
    f->server_id = field->server_id;
    f->group_id = field->group_id;
    f->server_group_id = field->server_group_id;
    f->title = field->title;
    f->is_hidden = field->is_hidden;
    f->deleted = field->deleted;
    f->timestamp_creation = field->timestamp_creation;
    f->user_id = logged_user->id;
    f->synchronized = false;
    auto id = view_field->persist(f);

    if (field->id < 0)
    {
        field->id = id;
    }
    else
    {
        field->id = f->id;
    }
    return READY;
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return ERROR;
}

pocket_stat_t pocket_field_controller_del(const pocket_field_controller_t* self, const pocket_field_t* field) try
{
    if (!self || !field) return ERROR;
    
    const auto view_field = static_cast<view<struct field> *>(self->view_field);

    view_field->del(field->id);

    return READY;
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return ERROR;
}

int32_t pocket_field_controller_size(const pocket_field_controller_t* self, pocket_stat_t group_id)
{

    return 0;
}

pocket_field_t* pocket_field_controller_get(const pocket_field_controller_t* self, pocket_stat_t group_id)
{

    return nullptr;
}

int32_t pocket_field_controller_count_child(const pocket_field_controller_t* self, const pocket_group_t* group) try
{
    if (self == nullptr || group == nullptr) return -1;
    const auto view_field = static_cast<view<struct field> *>(self->view_field);

    return view_field->get_list(group->id).size();
}
catch(const runtime_error& e)
{
    error(APP_TAG, e.what());
    return 0;
}
