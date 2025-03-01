```rs
h!(rect(), v!(rect(), h!(rect(), rect()))).width(20).height(10)

h!(
    rect(),
    v!(
        rect(),
        h!(
            rect(),
            rect(),
        )
    )
)

Horizontal {
    Rect1
    Vertical {
        Rect2
        Horizontal {
            Rect3
            Rect4
        }
    }
}

+-----------------+-----------------+
|                 |                 |
|                 |                 |
|                 |                 |
|                 |-----------------|
|                 |        |        |
|                 |        |        |
|                 |        |        |
+-----------------+--------+--------+

fn calc(x, y, width, height) {
    Horizontal {
        let nexpr = 2
        let area = Rect::new(x, y, width, height)
        let width = area.width / nexpr //10

        Rect1
        area.x += width //10

        Vertical {
            //Calculate the number of expressions in the Vertical container.
            let nexpr = 2
            let height = area.height / nexpr //5
            let area = Rect::new(area.x, area.y, width, height) //x 10, y 0, width 10, height 5

            Rect2
            area.y += height //5

            Horizontal {
                let nexpr = 2;
                let width = area.width / nexpr //5
                let area = Rect::new(area.x, area.y, width, height) //x 10, y 5, width 5, height 5

                Rect3
                area.x += width //15

                Rect4
                area.x += width //20
            }

            area.y += height //10
        }

        area.x += width //20
    }
}

calc(0, 0, 20, 10)
```
