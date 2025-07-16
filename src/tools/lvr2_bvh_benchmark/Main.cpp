/**
 * Copyright (c) 2025, University Osnabrück
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
#include <chrono>

#include <lvr2/io/ModelFactory.hpp>
#include <lvr2/geometry/BVH.hpp>
#include <lvr2/config/lvropenmp.hpp>
#include <lvr2/algorithm/raycasting/BVHRaycaster.hpp>
#include <lvr2/algorithm/raycasting/EmbreeRaycaster.hpp>

#include <boost/filesystem.hpp>
#include <boost/program_options.hpp>

using namespace lvr2;
using Clock = std::chrono::steady_clock;
using Vector = lvr2::BaseVector<float>;



int main(int argc, char** argv)
{
    ModelFactory io;
    auto model = io.readModel(argv[1]);

    if (!model->m_mesh)
    {
        std::cerr << "Could not load mesh from file: " << argv[1] << std::endl;
        return -1;
    }

    // Benchmark build time
    std::cout << "Building BVH now with " << lvr2::OpenMPConfig::getNumThreads() << " threads" << std::endl;
    auto t0 = Clock::now();
    BVHTree<Vector> tree(model->m_mesh);
    auto t1 = Clock::now();

    std::cout << "[BVH] Build time: " << std::chrono::duration_cast<std::chrono::milliseconds>(t1 - t0).count() << " ms" << std::endl;
    std::cout << "[BVH] Depth: " << tree.getMaxDepth() << std::endl;

    // Check that raycasting still works
    Vector3f origin = Vector3f::Zero();
    Vector3f dir = -Vector3f::UnitZ();
    AllInt intersection;

    // BVH Raycaster
    BVHRaycaster<AllInt> bvhraycaster(model->m_mesh);
    if (bvhraycaster.castRay(origin, dir, intersection))
    {
        std::cout << "[BVH] Intersection at t=" << intersection.dist << std::endl;
    }
    else
    {
        std::cout << "[BVH] No Intersection" << std::endl;
    }

    t0 = Clock::now();
    EmbreeRaycaster<AllInt> embreeraycaster(model->m_mesh);
    t1 = Clock::now();
    std::cout << "[Embree] BVH build: " << std::chrono::duration_cast<std::chrono::milliseconds>(t1 - t0).count() << " ms" << std::endl;
    if (embreeraycaster.castRay(origin, dir, intersection))
    {
        std::cout << "[Embree] Intersection at t=" << intersection.dist << std::endl;
    }
    else
    {
        std::cout << "[Embree] No Intersection" << std::endl;
    }


    // Benchmark raycasting
    // Directions for OS-128 like sensor 1024 x 128 with 45° fov
    std::vector<Vector3f> directions;
    directions.reserve(128 * 1024);
    for (int row = 0; row < 128; row++)
    {
        const float phi = -(M_PI / 8.0) + (row / 128.0) * M_PI_4;
        for (int col = 0; col < 1024; col++)
        {
            const float theta = (col / 1024.0) * M_2_PI;
            Vector3f dir;
            dir.x() = sin(phi) * cos(theta);
            dir.y() = sin(phi) * sin(theta);
            dir.z() = cos(phi);
            directions.push_back(dir.normalized());
        }
    }

    std::vector<uint8_t> hits;
    std::vector<AllInt> ints;

    t0 = std::chrono::steady_clock::now();
    for (int i = 0; i < 100; i++)
    {
        bvhraycaster.castRays(origin, directions, ints, hits);
    }
    t1 = std::chrono::steady_clock::now();
    const float sims_per_sec = 1e9 / (std::chrono::duration_cast<std::chrono::nanoseconds>(t1 - t0).count() / 100.0);
    std::cout << "Simulated " << sims_per_sec << " OS-128 per second" << std::endl;
}
