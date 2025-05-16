# ✍️ Exercises

Don't forget to checkout the branch:

```sh
git checkout 10-visual-effects
```

Let's review what was changed: <https://github.com/TheBevyFlock/bevy-quickstart-book/compare/09-sound-effects..10-visual-effects>

## Twinkle Twinkle Little Star

Stars twinkle, they don't stay at the same intensity all the time. Let's try to get that in our shader!

Tips:

- `globals.time` is available in the shader to do things that changes according to time
- Make a random number for star intensity that takes the time and the on screen position (so that all stars don't have the same intensity at the same time)
- multiply that value for each star layer

## More Star Variations

Make our star more diverse! Right now every run they have the same positions. Use our seeds to have different stars every time. And why not add some colors?

Tips:

- Use `material.seeds` with each star layer
- Multiply them by different colors

## New Ship

Let's add a shader displaying an effect when the a new ship is spawned.

Tips:

- Use the time the ship was spawned in the material
- Try to find a cool effect on <https://www.shadertoy.com> and port it
- If you want to modify the ship image, you'll need to create a new material and pass the image as a uniform
- If you want to display something hover the ship, you can make a simple material and display it with a higher z value
