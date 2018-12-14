/***************************************************************************
 *   Copyright (C) 2009 by Sindre Aamås                                    *
 *   aamas@stud.ntnu.no                                                    *
 *                                                                         *
 *   This program is free software; you can redistribute it and/or modify  *
 *   it under the terms of the GNU General Public License version 2 as     *
 *   published by the Free Software Foundation.                            *
 *                                                                         *
 *   This program is distributed in the hope that it will be useful,       *
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of        *
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the         *
 *   GNU General Public License version 2 for more details.                *
 *                                                                         *
 *   You should have received a copy of the GNU General Public License     *
 *   version 2 along with this program; if not, write to the               *
 *   Free Software Foundation, Inc.,                                       *
 *   59 Temple Place - Suite 330, Boston, MA  02111-1307, USA.             *
***************************************************************************/
#ifndef MINKEEPER_H
#define MINKEEPER_H

#include <algorithm>
#include "newstate.h"

namespace MinKeeperUtil {
template<int32_t n> struct CeiledLog2 { enum { R = 1 + CeiledLog2<(n + 1) / 2>::R }; };
template<> struct CeiledLog2<1> { enum { R = 0 }; };

template<int32_t v, int32_t n> struct RoundedDiv2n { enum { R = RoundedDiv2n<(v + 1) / 2, n - 1>::R }; };
template<int32_t v> struct RoundedDiv2n<v,1> { enum { R = v }; };

template<template<int> class T, int32_t n> struct Sum { enum { R = T<n-1>::R + Sum<T, n-1>::R }; };
template<template<int> class T> struct Sum<T,0> { enum { R = 0 }; };
}

// Keeps track of minimum value identified by id as values change.
// Higher ids prioritized (as min value) if values are equal. Can easily be reversed by swapping < for <=.
// Higher ids can be faster to change when the number of ids isn't a power of 2.
// Thus the ones that change more frequently should have higher ids if priority allows it.
template<int32_t ids>
class MinKeeper {
	enum { LEVELS = MinKeeperUtil::CeiledLog2<ids>::R };
	template<int32_t l> struct Num { enum { R = MinKeeperUtil::RoundedDiv2n<ids, LEVELS + 1 - l>::R }; };
	template<int32_t l> struct Sum { enum { R = MinKeeperUtil::Sum<Num, l>::R }; };
	
	template<int32_t id, int32_t level>
	struct UpdateValue {
		enum { P = Sum<level-1>::R + id };
		enum { C0 = Sum<level>::R + id * 2 };
		
		static void updateValue(MinKeeper<ids> &m) {
			// GCC 4.3 generates better code with the ternary operator on i386.
			m.a[P] = (id * 2 + 1 == Num<level>::R || m.values[m.a[C0]] < m.values[m.a[C0 + 1]]) ? m.a[C0] : m.a[C0 + 1];
			UpdateValue<id / 2, level - 1>::updateValue(m);
		}
	};
	
	template<int32_t id>
	struct UpdateValue<id,0> {
		static void updateValue(MinKeeper<ids> &m) {
			m.minValue_ = m.values[m.a[0]];
		}
	};
	
	class UpdateValueLut {
		template<int32_t id, int32_t dummy> struct FillLut {
			static void fillLut(UpdateValueLut & l) {
				l.lut_[id] = updateValue<id>;
				FillLut<id-1,dummy>::fillLut(l);
			}
		};
		
		template<int32_t dummy> struct FillLut<-1,dummy> {
			static void fillLut(UpdateValueLut &) {}
		};
		
		void (*lut_[Num<LEVELS-1>::R])(MinKeeper<ids>&);
		
	public:
		UpdateValueLut() { FillLut<Num<LEVELS-1>::R-1,0>::fillLut(*this); }
		void call(int32_t id, MinKeeper<ids> &mk) const { lut_[id](mk); }
	};
	
	static UpdateValueLut updateValueLut;
	uint32_t values[ids];
	uint32_t minValue_;
	int32_t a[Sum<LEVELS>::R];
	
	template<int32_t id> static void updateValue(MinKeeper<ids> &m);
	
public:
	explicit MinKeeper(uint32_t initValue = 0xFFFFFFFF);
	
	int32_t min() const { return a[0]; }
	uint32_t minValue() const { return minValue_; }
	
	template<int32_t id>
	void setValue(const uint32_t cnt) {
		values[id] = cnt;
		updateValue<id / 2>(*this);
	}
	
	void setValue(const int32_t id, const uint32_t cnt) {
		values[id] = cnt;
		updateValueLut.call(id >> 1, *this);
	}
	
	uint32_t value(const int32_t id) const { return values[id]; }

	// not sure if i understood everything in minkeeper correctly, so something might be missing here?
	template<bool isReader>
	void SyncState(gambatte::NewState *ns)
	{
		NSS(values);
		NSS(minValue_);
		NSS(a);
	}
};

template<int32_t ids> typename MinKeeper<ids>::UpdateValueLut MinKeeper<ids>::updateValueLut;

template<int32_t ids>
MinKeeper<ids>::MinKeeper(const uint32_t initValue) {
	std::fill(values, values + ids, initValue);
	
	for (int32_t i = 0; i < Num<LEVELS-1>::R; ++i) {
		a[Sum<LEVELS-1>::R + i] = (i * 2 + 1 == ids || values[i * 2] < values[i * 2 + 1]) ? i * 2 : i * 2 + 1;
	}
	
	int32_t n   = Num<LEVELS-1>::R;
	int32_t off = Sum<LEVELS-1>::R;
	
	while (off) {
		const int32_t pn = (n + 1) >> 1;
		const int32_t poff = off - pn;
		
		for (int32_t i = 0; i < pn; ++i) {
			a[poff + i] = (i * 2 + 1 == n ||
					values[a[off + i * 2]] < values[a[off + i * 2 + 1]]) ?
					a[off + i * 2] : a[off + i * 2 + 1];
		}
		
		off = poff;
		n   = pn;
	}
	
	minValue_ = values[a[0]];
}

template<int32_t ids>
template<int32_t id>
void MinKeeper<ids>::updateValue(MinKeeper<ids> &m) {
	m.a[Sum<LEVELS-1>::R + id] = (id * 2 + 1 == ids || m.values[id * 2] < m.values[id * 2 + 1]) ? id * 2 : id * 2 + 1;
	UpdateValue<id / 2, LEVELS-1>::updateValue(m);
}

#endif
