//シンプルなストップウォッチ

import kotlin.system.exitProcess

fun main() {
    println("===シンプルストップウォッチ===")
    println("コマンド：(s)スタート (p)一時停止 (r)リセット (q)終了\n")

    var isRunning = false
    var isPaused = false
    var startTime = 0L
    var pausedTime = 0L
    var elapsedTime = 0L

    val inputThread = Thread {
        while (true) {
            val input = readLine()?.trim()?.lowercase()

            when (input) {
                "s", "start" -> {
                    if (!isRunning) {
                        startTime = System.currentTimeMillis() - elapsedTime
                        isRunning = true
                        isPaused = false
                        println("▶️スタート")
                    } else if (isPaused) {
                        startTime = System.currentTimeMillis() - elapsedTime
                        isPaused = false
                        println("▶️再開")
                    }
                }

                "p", "pause" -> {
                    if (isRunning && !isPaused) {
                        pausedTime = System.currentTimeMillis()
                        elapsedTime = pausedTime - startTime
                        isPaused = true
                        println("⏸️一時停止")
                    }
                }

                "r", "reset" -> {
                    isRunning = false
                    isPaused = false
                    elapsedTime = 0L
                    println("↩️リセット")
                }

                "q", "quit" -> {
                    println("\n終了します")
                    exitProcess(0)
                }

                else -> {
                    println("無効なコマンドです")
                }
            }
        }
    }

    inputThread.isDaemon = true
    inputThread.start()

    while(true){
        if(isRunning && !isPaused){
            val currentTime = System.currentTimeMillis()
            elapsedTime = currentTime - startTime
        }

        val seconds = (elapsedTime / 1000) % 60
        val minutes = (elapsedTime / 60000) % 60
        val hours = elapsedTime / 3600000
        val millis = (elapsedTime % 1000) / 10

        print("\r⌚︎ %02d:%02d:%02d.%02d".format(hours, minutes, seconds, millis))

        Thread.sleep(10)
    }
}