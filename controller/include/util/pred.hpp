// Copyright (C) Roel Standaert
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#pragma once

namespace StandaertHA::Util::Pred {

  constexpr bool all_different() noexcept {
    return true;
  }

  template<typename T, typename... Types>
  constexpr bool all_different(T&& head, Types&&... tail) noexcept {
    return ((head != tail) && ...) && all_different(tail...);
  }

} // StandaertHA::Util
