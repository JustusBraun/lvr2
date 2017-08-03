/* Copyright (C) 2011 Uni Osnabrück
 * This file is part of the LAS VEGAS Reconstruction Toolkit,
 *
 * LAS VEGAS is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * LAS VEGAS is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA  02111-1307, USA
 */

/*
 * CleanupAlgorithms.hpp
 */

#ifndef LVR2_ALGORITHM_CLEANUPALGORITHMS_H_
#define LVR2_ALGORITHM_CLEANUPALGORITHMS_H_

#include <lvr2/geometry/BaseMesh.hpp>

namespace lvr2
{

/**
 * @brief Returns faces with a high number of boundary edges.
 *
 * Faces which have 2 or 3 adjacent boundary edges, are removed. If the face
 * is adjacent to only one boundary edge, it is deleted if the face's area is
 * smaller than `areaThreshold`.
 */
template<typename BaseVecT>
void cleanContours(BaseMesh<BaseVecT>& mesh, int iterations, float areaThreshold);

} // namespace lvr2

#include <lvr2/algorithm/CleanupAlgorithms.tcc>

#endif /* LVR2_ALGORITHM_CLEANUPALGORITHMS_H_ */
