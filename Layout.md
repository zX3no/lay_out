Relative/absolute positioning

```rs
//Default quadrant is top left, 20 units to the left
flex!(text()).x(20)

//20 units left and up from the bottom right corner.
flex!(text()).xy(20).relative(BottomRight)

//20 units left and down from the top right corner.
flex!(text()).xy(20).relative(TopRight)
```

Flex positioning

```rs
let t = text().wh(20);

//3 items will be next to each other.
flex!(t, t, t)

//The first item will start at x=0, then the second will start at x=30, 20 + 10, the third x=60, 30 + 20 + 10
flex!(t, t, t).gap(10)

//3 items will be spaced evenly across the parent viewport width.
flex!(t, t, t).hcenter()

//3 items will be spaced evenly across the parent viewport height.
flex!(t, t, t).vcenter()

//3 items will be spaced evenly across the parent viewport width and height.
flex!(t, t, t).vhcenter()
```

Flex direction

```rs
flex!(text()).vertical()
//or use the shorthand
v!(text())

flex!(text()).horizontal()
//or use the shorthand
h!(text())
```
