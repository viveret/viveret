---
date: 2014-11-17
title: Voxel Rendering Engine
image: /assets/images/projects/2014-11-17-23_04_27-screenshot.png
image_alt: Screenshot of a voxel 3D environment
description: >
  A voxel 3D rendering engine written in C++ using OpenGL.
tags: [ personal, computer-graphics, c++ ]
category: projects
layout: post
---

One of my hackathon ideas was to write a C++ voxel rendering engine and if successful use it in a future project. I feel like as a proof of concept or minimal viable product it fits the requirements. Any other changes would be making modifications to the voxel rendering engine. I did end up using it in future projects but the original hackathon code is lost to the sands of time.

## What is a Voxel?
>  A **voxel** (short for Volumetric Pixel) is the 3D equivalent of a 2D pixel.
> *   Think of a 2D image made of pixels.
> *   Imagine a series of these 2D grids so that the space between a grid is 1 unit in the Z direction. The squares now form tiny cubes, each existing at a specific X, Y, Z coordinate.
> *   Each of these tiny cubes is a voxel.

## What is a Voxel Engine?
> Voxel Engine is a type of graphics engine and game engine architecture built around manipulating and rendering these 3D grid cells (voxels). Instead of representing the world using complex polygon meshes (like triangles used in traditional 3D engines), the world is fundamentally represented as a collection of material cubes in a 3D grid.

## Tech Stack
C++ and OpenGL, some Lua

## Features Implemented
- Chunk rendering
- Viewport chunk occlusion
- Texturing and lighting
- Lua scripting to create scenes

## Room for Improvement
Lots of features and optimizations were left unimplemented. Well developed voxel engines and frameworks already exist with plenty of documentation and extensibility. If I were to redo this project, I would utilize and existing voxel engine and look for unique or novel ways to use it, perhaps with other existing libraries or technologies like physics or terrain simulation.