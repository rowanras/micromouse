
{ d[$2][$3] = $1-17 }

END {
    printf("Nav/Maze")
    for (n in d["100.maz"]) {
        printf(" %s", n);
    }
    printf("\n");
    for (m in d) {
        printf("%s", m);
        for (n in d["100.maz"]) {
            printf(" %d", d[m][n]);
        }
        printf("\n");
    }
}

