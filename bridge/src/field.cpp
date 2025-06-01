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

#include "pocket-pods/field.hpp"
#include "pocket/field.h"

#include <cstdlib>
#include <cstring>
#include <new>
#include <memory>

using namespace std;

pocket_field_t pocket_field_new()
{
 return {
  .id = 0,
  .server_id = 0,
  .user_id = 0,
  .group_id = 0,
  .server_group_id = 0,
  .group_field_id = 0,
  .server_group_field_id = 0,
  .title = nullptr,
  .value = nullptr,
  .is_hidden = false,
  .synchronized = false,
  .deleted = false,
  .timestamp_creation = 0
 };
}

pocket_field_t pocket_field_new_with_args(int64_t id, int64_t server_id, int64_t user_id, int64_t group_id,
 int64_t server_group_id, int64_t group_field_id, int64_t server_group_field_id, const char* title, const char* value,
 bool is_hidden,  bool synchronized, bool deleted, uint64_t timestamp_creation)
{
 pocket_field_t ret {
  .id = id,
  .server_id = server_id,
  .user_id = user_id,
  .group_id = group_id,
  .server_group_id = server_group_id,
  .group_field_id = group_field_id,
  .server_group_field_id = server_group_field_id,
  .is_hidden = is_hidden,
  .synchronized = synchronized,
  .deleted = deleted,
  .timestamp_creation = timestamp_creation
 };

 ret.title = new(nothrow) char[strlen(title)+1];
 if (ret.title == nullptr)
 {
  memset(&ret, '\0', sizeof(ret));
  return ret;
 }
 strcpy(ret.title, title);

 ret.value = new(nothrow) char[strlen(value)+1];;
 if (ret.value == nullptr)
 {
  memset(&ret, '\0', sizeof(ret));
  return ret;
 }
 strcpy(ret.value, value);

 return ret;
}

inline bool pocket_field_is_null(const pocket_field_t* field)
{
 return field != nullptr && field->title == nullptr;
}

void pocket_field_free(pocket_field_t* field)
{
 if (field == nullptr)
 {
  return;
 }

 if (field->title)
 {
  free(field->title);
  field->title = nullptr;
 }

 if (field->value)
 {
  free(field->value);
  field->value = nullptr;
 }

}


pocket::pods::field::ptr convert(const pocket_field_t* field)
{

 auto&& ret = make_unique<pocket::pods::field>();

 ret->id = field->id;
 ret->server_id = field->server_id;
 ret->user_id = field->user_id;
 ret->group_id = field->group_id;
 ret->server_group_id = field->server_group_id;
 ret->group_field_id = field->group_field_id;
 ret->server_group_field_id = field->server_group_field_id;
 if (field->title)
 {
  ret->title = field->title;
 }
 if (field->value)
 {
  ret->value = field->value;
 }
 ret->is_hidden = field->is_hidden;
 ret->synchronized = field->synchronized;
 ret->deleted = field->deleted;
 ret->timestamp_creation = field->timestamp_creation;

 return ret;
}

pocket_field_t convert(const pocket::pods::field::ptr &field)
{
 pocket_field_t ret;
 memset(&ret, 0, sizeof(ret));

 ret.id = field->id;
 ret.server_id = field->server_id;
 ret.user_id = field->user_id;
 ret.group_id = field->group_id;
 ret.server_group_id = field->server_group_id;
 ret.group_field_id = field->group_field_id;
 ret.server_group_field_id = field->server_group_field_id;

 ret.title = new(nothrow) char[field->title.size() + 1];
 if (ret.title == nullptr)
 {
  memset(&ret, '\0', sizeof(ret));
  return ret;
 }
 strcpy(ret.title, field->title.c_str());

 ret.value = new(nothrow) char[field->value.size() + 1];
 if (ret.value == nullptr)
 {
  memset(&ret, '\0', sizeof(ret));
  return ret;
 }
 strcpy(ret.value, field->value.c_str());

 ret.is_hidden = field->is_hidden;
 ret.synchronized = field->synchronized;
 ret.deleted = field->deleted;
 ret.timestamp_creation = field->timestamp_creation;

 return ret;
}
