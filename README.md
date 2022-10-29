## Epicycles Renderer
This simple tool takes a list of epicycles and renders approximation of shape that last of the epicycle follows in time.

Epicycles are passed as path to csv file where every epicycle is defined as tuple of angular speed, radius and initial angle.

```
cargo run -- --input example/fish.csv --output fish.png
```

In-depth animated & interactive explanation at [myFourierEpicycles](https://www.myfourierepicycles.com/) (And way more fun! Really, check it out!)

Inspired by [InterLOS #2016](htps://interlos.fi.muni.cz/download/years/2016) (only in czech).
