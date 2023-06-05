import java.io.File
import java.nio.file.Paths

fun readInput(): String {
    return File("src/y2015/day1-input.txt").readText()
}

fun q1(): Int = readInput().asIterable().fold(0) {acc, c -> acc + if (c == '(') {1} else {-1}}

println("Day1-Q1: ${q1()}")

fun q2(): Int {
    val text = readInput()
    var floor = 0
    for (i in 0 until text.length) {
        floor += if (text[i] == '(') {1} else {-1}
        if (floor == -1) return i + 1
    }

    return -1
}

println("Day1-Q2: ${q2()}")