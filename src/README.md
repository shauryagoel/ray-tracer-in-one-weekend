# Introduction

This document contains concepts defined and introduced in the [blog](https://raytracing.github.io/books/RayTracingInOneWeekend.html) for many chapters and sections.

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

### Section 4.2

A Ray Tracer basically sends rays through pixels and computes the color seen in the direction of those rays. There are 3 steps are-

1. Calculate the ray from the "eye" (camera) through the pixel
2. Determine which objects the ray intersects
3. Compute a color for the closest intersection point

```math
aspect\_ratio = \frac{width}{height}
```

We can keep the `aspect_ratio` fixed, and vary the `width`, to get the `height`. This way we can scale up or down the image. We must ensure that the new height is at least 1.

**Viewport**: It is a virtual rectangle in the 3D world that contains the grid of image pixel locations.

If pixels are spaced the same distance horizontally as they are vertically, the viewport that bounds it will have the same aspect ratio as the rendered image. The distance between two adjacent pixels is called **Pixel Spacing**. Square pixels are the standard.

**Camera** (eye): A point in 3D space from which all scene rays will originate.

**Focal Length**: The distance between the camera center point and the viewport center point.

The vector from the camera center to the viewport center will be orthogonal to the viewport center.

We will keep the camera at the origin and viewport at $z=-1$ and the height of viewport to be of 2 units.

![A description of the image](https://raytracing.github.io/images/fig-1.04-pixel-grid.jpg)

## Chapter 5

### Section 5.1

Equation for a sphere with center $𝐂 = (C_x, C_y, C_z)$, radius $r$, and an arbitrary point $𝐏 = (x, y, z)$ is-

```math
(𝐂 - 𝐏) \cdot (𝐂 - 𝐏) = (C_x - x)^2 + (C_y - y)^2 + (C_z - z)^2 = r^2
```

To find if a ray, $𝐏(t) = 𝐐 + t \cdot 𝐝$, hits the sphere-

$$
\begin{gathered}
(𝐂 - (𝐐 + t𝐝)) \cdot (𝐂 - (𝐐 + t𝐝)) = r^2 \\
\implies t^2 𝐝 \cdot 𝐝 - 2t 𝐝 \cdot (𝐂 - 𝐐) + (𝐂 - 𝐐) \cdot (𝐂 - 𝐐) - r^2 = 0
\end{gathered}
$$

This can be seen as a quadratic equation, $ax^2 + bx + c = 0$, which can be solved using the quadratic formula-

```math
\dfrac{-b \pm \sqrt{b^2 - 4ac}}{2a}
```

Using this to solve for $t$ would require the following values to be substituted to the quadratic formula-

$$
\begin{aligned}
a &= 𝐝 \cdot 𝐝 \\
b &= -2𝐝 \cdot (𝐂 - 𝐐) \\
c &= (𝐂 - 𝐐) \cdot (𝐂 - 𝐐) - r^2
\end{aligned}
$$

The square root part in the quadratic formula can be-

- positive: meaning two real solutions - ray passes in between the sphere
- negative: meaning no real solution - ray does not touch the sphere
- zero: meaning one real solution - ray is tangent to the sphere
