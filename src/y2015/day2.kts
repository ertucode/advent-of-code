import java.io.File
import java.io.InputStream
import java.io.BufferedReader

data class Prism(val l: Int, val w: Int, val h: Int) {
    fun q1Area(): Int {
        val smallest = smallestTwo()
        return 2 * this.l * this.w + 2 * this.w * this.h + 2 * this.h * this.l + smallest.first * smallest.second
    }

    fun smallestTwo(): Pair<Int, Int> {
        var arr = intArrayOf(this.l, this.w, this.h)
        arr.sort()
        return Pair(arr[0], arr[1])
    }
}

fun q1(): Int {
    val file = File("day2-input.txt")
    var res: Int = 0
    file.useLines{lines -> lines.forEach {
        val splits = it.split("x").map { it.toInt() }
        val prism = Prism(splits[0], splits[1], splits[2])
        res += prism.q1Area()
    }}
    return res
}

println("Day2 - Q1: ${q1()}")
