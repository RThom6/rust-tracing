# Integrating Light Falling around a Pixel

The simplest model for representing a pixel is to sample the square region centred at the pixel that extends halfway to all of its neighbouring pixels. Very straightforward but nowhere near optimal. Two separate ways of modelling a pixel as a square are to either have 0,0 in the center, or 0,0 in the top-left corner. The area bounded by the square thusly become {(x, y) | i-.5 ≤ x ≤ i+.5, j-.5 ≤ y ≤ j+.5} or {(x, y) | i ≤ x ≤ i+1., j ≤ y ≤ j+1.}. 

## What is a Pixel?

A pixel is a point sample which exists only at a point. For a colour picture, the pixel may actually have 3 samples, one for each primary colour. It can still be thought of as a point sample of colour, but not as a square or anything that isn't a point. An image is a rectilinear array (elements are ordered in a straight, grid-like pattern aligned with orthogonal axes) of point samples which according to the Sampling Theorm we can reconstruct a continuous entity from with a reconstruction filter. If you wanted to associate a pixel with a shape, then the best bet would be the footprint of the reconstruction filter you're using --book says probably not to associate a shape at all--. Representing a pixel as a little square around the pixel would cause jagged edges and lead to a low quality resolution. 

The edges of a reconstructed image can be defined as its minimally enclosing rectangle.

Seeing square pixels when zooming in a video is a result of each point sample being replicated MxM times. To show what would actually happen on zooming the image would require a resampling operation which was too slow to be comfortably supported by real time video at the time of the paper (1995) -> read up on this later for today's cards.
