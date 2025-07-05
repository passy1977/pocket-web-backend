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
#include "pocket-bridge/field.h"

#include <cstdlib>
#include <cstring>
#include <new>
#include <memory>

using namespace std;

pocket_field_t* pocket_field_new()
{
 auto field = new(nothrow) pocket_field_t{
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
 if (field == nullptr) return nullptr;

 return field;
}

pocket_field_t* pocket_field_new_with_args(int64_t id, int64_t server_id, int64_t user_id, int64_t group_id,
 int64_t server_group_id, int64_t group_field_id, int64_t server_group_field_id, const char* title, const char* value,
 bool is_hidden,  bool synchronized, bool deleted, uint64_t timestamp_creation)
{
 auto field = pocket_field_new();
 if (field == nullptr) return nullptr;

 field->id = id;
 field->server_id = server_id;
 field->user_id = user_id;
 field->group_id = group_id;
 field->server_group_id = server_group_id;
 field->group_field_id = group_field_id;
 field->server_group_field_id = server_group_field_id;

 field->title = strdup(title);
 if (field->title == nullptr) return nullptr;

 field->value = strdup(value);
 if (field->value == nullptr)
 {
  pocket_field_free(field);
  return field;
 }

 field->is_hidden = is_hidden;
 field->synchronized = synchronized;
 field->deleted = deleted;
 field->timestamp_creation = timestamp_creation;

 return field;
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
  delete  field->title;
  field->title = nullptr;
 }

 if (field->value)
 {
  delete field->value;
  field->value = nullptr;
 }

 delete field;

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
