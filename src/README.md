# Introduction

This document contains concepts defined and introduced in the [blog](https://raytracing.github.io/books/RayTracingInOneWeekend.html) separated chapter and section wise.

## Chapter 2

### Section 2.1

**PPM format**: PPM is a format to store images just like JPG and PNG are other formats.

E.g.-

```ppm
P3
1 2
255
255 0 0
0 255 0
```

Description-

1) P3: Indicates that our image is in ASCII

2) 1 2: Indicates that our image has 1 column and 2 rows

3) 255: Indicates that **255** is the max color value

4) RGB values: Rest of the file contains RGB values at all locations in the image, starting from left to right and top to bottom. These values are integers between **0** and **255**.

## Chapter 3

**Vec3** struct is created which is used to hold 3 float64 values. This struct can be used to represent colors, locations, directions, offsets, etc.

## Chapter 4

### Section 4.1

**Ray**: A line can be represented as-

$$
\mathbf{P}(t) = \mathbf{A} + t \mathbf{b}
$$

$$
\begin{aligned}
\text{where,} \quad & \mathbf{P} && \text{is a 3D position along a line in 3D} \\
                     & \mathbf{b} && \text{is the ray direction} \\
                     & \mathbf{t} && \text{is a real number (double)}
\end{aligned}
$$

For different values of $t$, we can move along the line-

- If $t$ is allowed to go to all real values (positive and negative), then $\mathbf{P}$ represents a line

- If $t$ is allowed to be only positive, then $\mathbf{P}$ represents a ray
