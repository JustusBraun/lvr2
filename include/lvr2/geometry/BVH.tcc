/**
 * Copyright (c) 2018, University Osnabrück
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *     * Neither the name of the University Osnabrück nor the
 *       names of its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL University Osnabrück BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/*
 * BVH.tcc
 *
 *  @date 21.01.2018
 *  @author Johan M. von Behren <johan@vonbehren.eu>
 */

#include <limits>

using std::make_unique;
using std::transform;
using std::move;

namespace lvr2
{

template<typename BaseVecT>
BVHTree<BaseVecT>::Triangle::Triangle()
    : idx1(0)
    , idx2(0)
    , idx3(0)
    , center(0, 0, 0)
    , normal(0, 1, 0)
    , d(0.0f)
    , d1(0.0f)
    , d2(0.0f)
    , d3(0.0f)
    , e1(0, 1, 0)
    , e2(0, 1, 0)
    , e3(0, 1, 0)
    , bb()
{}

template<typename BaseVecT>
BVHTree<BaseVecT>::BVHTree(const vector<float>& vertices, const vector<uint32_t>& faces)
: m_depth(0)
{
    m_root = buildTree(vertices, faces);
    createCFTree();
}

template<typename BaseVecT>
BVHTree<BaseVecT>::BVHTree(
    const floatArr vertices, size_t n_vertices,
    const indexArray faces, size_t n_faces)
: m_depth(0)
{
    m_root = buildTree(vertices, n_vertices, faces, n_faces);
    createCFTree();
}

template<typename BaseVecT>
BVHTree<BaseVecT>::BVHTree(const MeshBufferPtr mesh)
:BVHTree<BaseVecT>(
    mesh->getVertices(), mesh->numVertices(),
    mesh->getFaceIndices(), mesh->numFaces()
    )
{

}

template<typename BaseVecT>
typename BVHTree<BaseVecT>::BVHNodePtr BVHTree<BaseVecT>::buildTree(
    const vector<float>& vertices,
    const vector<uint32_t>& faces
)
{
    vector<AABB> work;
    work.reserve(faces.size() / 3);
    m_triangles.reserve(faces.size() / 3);

    BoundingBox<BaseVecT> outerBb;

    // Iterate over all faces and create an AABB for all of them
    for (size_t i = 0; i < faces.size(); i += 3)
    {
        // Convert raw float data into objects
        BaseVecT point1;
        point1.x = vertices[faces[i]*3];
        point1.y = vertices[faces[i]*3+1];
        point1.z = vertices[faces[i]*3+2];

        BaseVecT point2;
        point2.x = vertices[faces[i+1]*3];
        point2.y = vertices[faces[i+1]*3+1];
        point2.z = vertices[faces[i+1]*3+2];

        BaseVecT point3;
        point3.x = vertices[faces[i+2]*3];
        point3.y = vertices[faces[i+2]*3+1];
        point3.z = vertices[faces[i+2]*3+2];

        // Precalculate intersection test data for faces
        auto vc1 = point2 - point1;
        auto vc2 = point3 - point2;
        auto vc3 = point1 - point3;

        // skip malformed faces
        auto cross1 = vc1.cross(vc2);
        auto cross2 = vc2.cross(vc3);
        auto cross3 = vc3.cross(vc1);
        if (cross1.length() == 0 || cross2.length() == 0 || cross3.length() == 0)
        {
            continue;
        }

        BoundingBox<BaseVecT> faceBb;
        faceBb.expand(point1);
        faceBb.expand(point2);
        faceBb.expand(point3);

        // Create triangles from faces for internal usage
        Triangle triangle;
        triangle.bb = faceBb;
        triangle.center = BaseVecT((point1.asVector() + point2.asVector() + point3.asVector()) / 3.0f);
        triangle.idx1 = faces[i];
        triangle.idx2 = faces[i+1];
        triangle.idx3 = faces[i+2];

        // pick best normal
        Normal<typename BaseVecT::CoordType> normal1(cross1);
        Normal<typename BaseVecT::CoordType> normal2(cross2);
        Normal<typename BaseVecT::CoordType> normal3(cross3);
        auto bestNormal = normal1;
        if (normal2.length() > bestNormal.length())
        {
            bestNormal = normal2;
        }
        if (normal3.length() > bestNormal.length())
        {
            bestNormal = normal3;
        }
        triangle.normal = Normal<typename BaseVecT::CoordType>(bestNormal);

        triangle.d = triangle.normal.dot(point1);

        // calc edge planes for intersection tests
        triangle.e1 = Normal<typename BaseVecT::CoordType>(triangle.normal.cross(vc1));
        triangle.d1 = triangle.e1.dot(point1);

        triangle.e2 = Normal<typename BaseVecT::CoordType>(triangle.normal.cross(vc2));
        triangle.d2 = triangle.e2.dot(point2);

        triangle.e3 = Normal<typename BaseVecT::CoordType>(triangle.normal.cross(vc3));
        triangle.d3 = triangle.e3.dot(point3);

        // Create AABB and add current triangle (face) to it
        AABB aabb;
        aabb.bb = faceBb;
        aabb.triangle = m_triangles.size();
        m_triangles.push_back(triangle);

        outerBb.expand(faceBb);
        work.push_back(aabb);
    }

    // Create the tree recursively from the list of AABBs

    BVHTree<BaseVecT>::BVHNodePtr out;

    #pragma omp parallel
    #pragma omp single nowait
    out = buildTreeRecursive(work.begin(), work.end());
    
    std::cout << "end building." << std::endl;
    out->bb = outerBb;

    return out;
}


template<typename BaseVecT>
typename BVHTree<BaseVecT>::BVHNodePtr BVHTree<BaseVecT>::buildTree(
    const floatArr vertices, size_t n_vertices,
    const indexArray faces, size_t n_faces
)
{
    vector<AABB> work;
    work.reserve(n_faces);
    m_triangles.reserve(n_faces);

    BoundingBox<BaseVecT> outerBb;
    // Iterate over all faces and create an AABB for all of them
    for (size_t i = 0; i < n_faces*3; i += 3)
    {
        // Convert raw float data into objects
        BaseVecT point1;
        point1.x = vertices[faces[i]*3];
        point1.y = vertices[faces[i]*3+1];
        point1.z = vertices[faces[i]*3+2];

        BaseVecT point2;
        point2.x = vertices[faces[i+1]*3];
        point2.y = vertices[faces[i+1]*3+1];
        point2.z = vertices[faces[i+1]*3+2];

        BaseVecT point3;
        point3.x = vertices[faces[i+2]*3];
        point3.y = vertices[faces[i+2]*3+1];
        point3.z = vertices[faces[i+2]*3+2];

        // Precalculate intersection test data for faces
        auto vc1 = point2 - point1;
        auto vc2 = point3 - point2;
        auto vc3 = point1 - point3;

        // skip malformed faces
        auto cross1 = vc1.cross(vc2);
        auto cross2 = vc2.cross(vc3);
        auto cross3 = vc3.cross(vc1);
        if (cross1.length() == 0 || cross2.length() == 0 || cross3.length() == 0)
        {
            continue;
        }

        BoundingBox<BaseVecT> faceBb;
        faceBb.expand(point1);
        faceBb.expand(point2);
        faceBb.expand(point3);

        // Create triangles from faces for internal usage
        Triangle triangle;
        triangle.bb = faceBb;
        triangle.center = (point1 + point2 + point3) / 3.0f;
        triangle.idx1 = faces[i];
        triangle.idx2 = faces[i+1];
        triangle.idx3 = faces[i+2];

        // pick best normal
        typename BaseVecT::CoordType test = 2.0;
        Normal<typename BaseVecT::CoordType> normal1(cross1);
        Normal<typename BaseVecT::CoordType> normal2(cross2);
        Normal<typename BaseVecT::CoordType> normal3(cross3);
        auto bestNormal = normal1;
        if (normal2.length() > bestNormal.length())
        {
            bestNormal = normal2;
        }
        if (normal3.length() > bestNormal.length())
        {
            bestNormal = normal3;
        }
        triangle.normal = Normal<typename BaseVecT::CoordType>(bestNormal);

        triangle.d = triangle.normal.dot(point1);

        // calc edge planes for intersection tests
        triangle.e1 = Normal<typename BaseVecT::CoordType>(triangle.normal.cross(vc1));
        triangle.d1 = triangle.e1.dot(point1);

        triangle.e2 = Normal<typename BaseVecT::CoordType>(triangle.normal.cross(vc2));
        triangle.d2 = triangle.e2.dot(point2);

        triangle.e3 = Normal<typename BaseVecT::CoordType>(triangle.normal.cross(vc3));
        triangle.d3 = triangle.e3.dot(point3);

        // Create AABB and add current triangle (face) to it
        AABB aabb;
        aabb.bb = faceBb;
        aabb.triangle = m_triangles.size();
        m_triangles.push_back(triangle);

        outerBb.expand(faceBb);
        work.push_back(aabb);
    }


    // Create the tree recursively from the list of AABBs
    BVHTree<BaseVecT>::BVHNodePtr out;
    #pragma omp parallel
    #pragma omp single nowait
    out = buildTreeRecursive(work.begin(), work.end());

    out->bb = outerBb;

    return out;
}

template<typename BaseVecT>
typename BVHTree<BaseVecT>::BVHNodePtr BVHTree<BaseVecT>::buildTreeRecursive(
    typename vector<AABB>::iterator work_begin,
    typename vector<AABB>::iterator work_end,
    uint32_t depth
)
{
    // The number of buckets to test during best split computation
    constexpr const int BUCKETS = 32;
    // Determine the bounding box of this node
    BoundingBox<BaseVecT> bb;
    for (auto it = work_begin; it != work_end; it++)
    {
        bb.expand(it->bb);
    }

    // terminate recursion, if work size is small enough
    if (std::distance(work_begin, work_end) <= 4)
    {
        // Create a leaf node and add all remaining triangles into it
        auto leaf = make_unique<BVHLeaf>();
        leaf->bb = bb;
        leaf->triangles.reserve(std::distance(work_begin, work_end));

        for (auto aabb = work_begin; aabb != work_end; aabb++)
        {
            leaf->triangles.push_back(aabb->triangle);
        }
        #pragma omp critical(bvh_depth_update)
        {
            m_depth = std::max(m_depth, depth);
        }
        return move(leaf);
    }

    // divide node into smaller nodes
    // SAH, surface area heuristic calculation
    float minCost =
        std::distance(work_begin, work_end) * (bb.getXSize() * bb.getYSize() + bb.getYSize() * bb.getZSize() + bb.getZSize() * bb.getXSize());

    int bestSplitIdx = -1;
    int bestAxis = -1;

    // try all 3 axises X = 0, Y = 1, Z = 2
    for (uint8_t axis = 0; axis < 3; axis++)
    {
        float start, stop, step;

        if (axis == 0)
        {
            start = bb.getMin().x;
            stop = bb.getMax().x;
        }
        else if(axis == 1)
        {
            start = bb.getMin().y;
            stop = bb.getMax().y;
        }
        else
        {
            start = bb.getMin().z;
            stop = bb.getMax().z;
        }

        if (fabs(stop - start) < 1e-4)
        {
            // bb side along this axis too short, we must move to a different axis
            continue;
        }

        BoundingBox<BaseVecT> buckets[BUCKETS];
        int counts[BUCKETS] = {};

        step = (stop - start) / BUCKETS;
        const float range = stop - start;
        // Sort bbs into buckets
        for (auto v = work_begin; v != work_end; v++)
        {
            float value;
            if (axis == 0)
            {
                value = v->bb.getCentroid().x;
            }
            else if (axis == 1)
            {
                value = v->bb.getCentroid().y;
            }
            else
            {
                value = v->bb.getCentroid().z;
            }

            const int index = std::clamp<int>(std::floor((value - start) * (BUCKETS) / range), 0, BUCKETS - 1);
            buckets[index].expand(v->bb);
            counts[index]++;
        }

        // Check all split positions
        for (int i = 1; i < BUCKETS; i++)
        {
            const float testSplit = start + i * step;
            BoundingBox<BaseVecT> lBb;
            BoundingBox<BaseVecT> rBb;

            int countLeft = 0, countRight = 0;
            for (int j = 0; j < i; j++)
            {
                if (!buckets[j].isValid())
                {
                    continue;
                }
                lBb.expand(buckets[j]);
                countLeft += counts[j];
            }

            for (int j = i; j < BUCKETS; j++)
            {
                if (!buckets[j].isValid())
                {
                    continue;
                }
                rBb.expand(buckets[j]);
                countRight += counts[j];
            }

            if (countLeft <= 1 || countRight <= 1 || !lBb.isValid() || !rBb.isValid())
            {
                continue;
            }

            // Calc surface for left and right box
            float surfaceLeft =
                lBb.getXSize() * lBb.getYSize() + lBb.getYSize() * lBb.getZSize() + lBb.getZSize() * lBb.getXSize();
            float surfaceRight =
                rBb.getXSize() * rBb.getYSize() + rBb.getYSize() * rBb.getZSize() + rBb.getZSize() * rBb.getXSize();

            // Check if new best split was found
            float totalCost = surfaceLeft * countLeft + surfaceRight * countRight;
            if (totalCost < minCost)
            {
                minCost = totalCost;
                bestSplitIdx = i;
                bestAxis = axis;
            }
        }
    }

    // If no good split was found, create a leaf node and copy all remaining triangles into it
    if (bestAxis == -1)
    {
        auto leaf = make_unique<BVHLeaf>();
        leaf->bb = bb;
        leaf->triangles.reserve(std::distance(work_begin, work_end));

        for (auto aabb = work_begin; aabb != work_end; aabb++)
        {
            leaf->triangles.push_back(aabb->triangle);
        }
        #pragma omp critical(bvh_depth_update)
        {
            m_depth = std::max(m_depth, depth);
        }
        return move(leaf);
    }

    // Use the found split to split the current node into two new inner nodes
    float range = -1.0;
    float start = 0.0;
    
    switch(bestAxis)
    {
        case 0:
            range = bb.getMax().x - bb.getMin().x;
            start = bb.getMin().x;
            break;
        case 1:
            range = bb.getMax().y - bb.getMin().y;
            start = bb.getMin().y;
            break;
        default:
            range = bb.getMax().z - bb.getMin().z;
            start = bb.getMin().z;
            break;
    }

    auto pred = [bestAxis, bestSplitIdx, range, start](const auto& a) -> bool
    {
        float value = 0.0;
        switch (bestAxis)
        {
            case 0: value = a.bb.getCentroid().x; break;
            case 1: value = a.bb.getCentroid().y; break;
            default: value = a.bb.getCentroid().z; break;
        }
        const int index = std::clamp<int>(std::floor((value - start) * (BUCKETS) / range), 0, BUCKETS - 1);
        return index < bestSplitIdx;
    };
    auto partition_point = std::partition(work_begin, work_end, pred);
    
    // Recursively split new sub trees into further inner or leaf nodes
    auto inner = make_unique<BVHInner>();
    inner->bb = bb;

    // Spawn the left child as a task
    #pragma omp task shared(inner)
    inner->left = buildTreeRecursive(work_begin, partition_point, depth + 1);

    // Execute the right tree in this thread
    inner->right = buildTreeRecursive(partition_point, work_end, depth + 1);
    #pragma omp taskwait
    
    return move(inner);
}

template<typename BaseVecT>
void BVHTree<BaseVecT>::createCFTree()
{
    m_triIndexList.reserve(m_triangles.size());
    uint32_t idxBoxes = 0;
    createCFTreeRecursive(move(m_root), idxBoxes);
    convertTrianglesIntersectionData();
}

template<typename BaseVecT>
void BVHTree<BaseVecT>::createCFTreeRecursive(BVHNodePtr currentNode, uint32_t& idxBoxes)
{
    // Convert bounding box limits to SIMD friendly format
    m_limits.push_back(currentNode->bb.getMin().x);
    m_limits.push_back(currentNode->bb.getMax().x);

    m_limits.push_back(currentNode->bb.getMin().y);
    m_limits.push_back(currentNode->bb.getMax().y);

    m_limits.push_back(currentNode->bb.getMin().z);
    m_limits.push_back(currentNode->bb.getMax().z);

    // If we have an inner node
    if (!currentNode->isLeaf())
    {
        BVHInnerPtr inner(dynamic_cast<BVHInner*>(currentNode.release()));

        // push dummy count (0 -> inner node!)
        m_indexesOrTrilists.push_back(0);

        // push dummy box indices (they will be fixed later)
        size_t indexesOrTrilistsPos = m_indexesOrTrilists.size();
        m_indexesOrTrilists.push_back(0);
        m_indexesOrTrilists.push_back(0);

        // push dummy start index
        m_indexesOrTrilists.push_back(0);

        // now recurse
        uint32_t idxLeft = ++idxBoxes;
        createCFTreeRecursive(move(inner->left), idxBoxes);
        uint32_t idxRight = ++idxBoxes;
        createCFTreeRecursive(move(inner->right), idxBoxes);

        // fix box indices
        m_indexesOrTrilists[indexesOrTrilistsPos] = idxLeft;
        m_indexesOrTrilists[indexesOrTrilistsPos + 1] = idxRight;
    }
    else
    {
        // If we have a leaf node
        BVHLeafPtr leaf(dynamic_cast<BVHLeaf*>(currentNode.release()));
        uint32_t count = static_cast<uint32_t>(leaf->triangles.size());

        // push real count
        m_indexesOrTrilists.push_back(0x80000000 | count);

        // push dummy box indices
        m_indexesOrTrilists.push_back(0);
        m_indexesOrTrilists.push_back(0);

        // push start index
        m_indexesOrTrilists.push_back(static_cast<uint32_t>(m_triIndexList.size()));

        // copy triangle indices
        for (auto idx: leaf->triangles)
        {
            m_triIndexList.push_back(static_cast<uint32_t>(idx));
        }
    }
}

template<typename BaseVecT>
void BVHTree<BaseVecT>::convertTrianglesIntersectionData()
{
    uint32_t sizePerTriangle = 4 + 4 + 4 + 4;
    m_trianglesIntersectionData.reserve(m_triangles.size() * sizePerTriangle);
    for (auto const& triangle: m_triangles)
    {
        m_trianglesIntersectionData.push_back(triangle.normal.getX());
        m_trianglesIntersectionData.push_back(triangle.normal.getY());
        m_trianglesIntersectionData.push_back(triangle.normal.getZ());
        m_trianglesIntersectionData.push_back(triangle.d);

        m_trianglesIntersectionData.push_back(triangle.e1.getX());
        m_trianglesIntersectionData.push_back(triangle.e1.getY());
        m_trianglesIntersectionData.push_back(triangle.e1.getZ());
        m_trianglesIntersectionData.push_back(triangle.d1);

        m_trianglesIntersectionData.push_back(triangle.e2.getX());
        m_trianglesIntersectionData.push_back(triangle.e2.getY());
        m_trianglesIntersectionData.push_back(triangle.e2.getZ());
        m_trianglesIntersectionData.push_back(triangle.d2);

        m_trianglesIntersectionData.push_back(triangle.e3.getX());
        m_trianglesIntersectionData.push_back(triangle.e3.getY());
        m_trianglesIntersectionData.push_back(triangle.e3.getZ());
        m_trianglesIntersectionData.push_back(triangle.d3);
    }
}

template<typename BaseVecT>
const vector<uint32_t>& BVHTree<BaseVecT>::getTriIndexList() const
{
    return m_triIndexList;
}

template<typename BaseVecT>
const vector<float>& BVHTree<BaseVecT>::getLimits() const
{
    return m_limits;
}

template<typename BaseVecT>
const vector<uint32_t>& BVHTree<BaseVecT>::getIndexesOrTrilists() const
{
    return m_indexesOrTrilists;
}

template<typename BaseVecT>
const vector<float>& BVHTree<BaseVecT>::getTrianglesIntersectionData() const
{
    return m_trianglesIntersectionData;
}

template<typename BaseVecT>
const uint32_t BVHTree<BaseVecT>::getMaxDepth() const noexcept
{
    return m_depth;
}

} /* namespace lvr2 */
