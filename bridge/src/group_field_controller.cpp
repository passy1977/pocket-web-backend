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

#include "pocket-bridge/group_field_controller.h"

#include "pocket/globals.hpp"
using namespace pocket;

#include "pocket-controllers/session.hpp"
using controllers::session;

#include "pocket-views/view-group-field.hpp"
using views::view;

#include "pocket-pods/group-field.hpp"
using pods::group_field;

#include <new>
#include <ranges>
using namespace std;

extern pocket_group_field_t* convert(const group_field::ptr& group);

pocket_group_field_controller_t* pocket_group_field_controller_new(pocket_t* pocket)
{
    if (pocket == nullptr)
    {
        return nullptr;
    }
    return new(nothrow) pocket_group_field_controller_t {
        .pocket = pocket,
        .reachability = true,
        .view_group_field = nullptr,
    };
}

void pocket_group_field_controller_free(const pocket_group_field_controller_t* self)
{
    if (self == nullptr)
    {
        return;
    }

    delete self;
}

void pocket_group_field_controller_init(pocket_group_field_controller_t* self)
{
    if (self && self->pocket && self->pocket->session)
    {
        auto session = static_cast<class session*>(self->pocket->session);
        self->view_group_field = session->get_view_group_field().get();
    }
}

pocket_group_field_t** pocket_group_field_controller_get_list(const pocket_group_field_controller_t* self, int64_t group_id, const char *search, int *count)
{
    if (self == nullptr || search == nullptr || count == nullptr)
    {
        return nullptr;
    }

    const auto view_group_field = static_cast<view<struct group_field> *>(self->view_group_field);

    auto&& list = view_group_field->get_list(group_id, search);
    *count = static_cast<int>(list.size());

    const auto ret = new(nothrow) pocket_group_field_t*[*count];
    if (ret == nullptr)
    {
        return nullptr;
    }

    size_t i = 0;
    for(auto &&it : list)
    {
        const auto pocket_group_field = convert(it);

        ret[i]  = pocket_group_field;
        i++;
    }

    return ret;
}

void pocket_group_field_controller_free_list(pocket_group_field_t** list, int count)
{
    if (list == nullptr)
    {
        return;
    }

    for (int i : std::views::iota(0, count)) {
        pocket_group_field_free(list[i]);
    }

    delete [] list;
}
