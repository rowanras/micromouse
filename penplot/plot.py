from blackstripes import crossed

print("Running")

IMAGE = "IMG_1257"

crossed.draw(
    "./images/" + IMAGE + ".png",       # input file
    "./output.svg",                     # output file
    1.0,                                # nibsize
    "#333333",                          # line color
    1,                                  # scale
    200, 150, 100, 50,                  # levels
    1,                                  # type
    540, 1021, 0.7                      # signature transform
)

