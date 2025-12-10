//シンプルTodoリスト

fun main(){
    val todos = mutableListOf<String>()
    println("===簡易Todoリスト===")

    while(true){
        println("\nコマンドを選んでください：(1)追加 (2)一覧 (3)削除 (4)終了")
        println("> ")
        val input = readLine()?.trim()

        when(input){
            "1", "追加" -> addTodo(todos)
            "2", "一覧" -> listTodos(todos)
            "3", "削除" -> removeTodo(todos)
            "4", "終了" -> {
                println("お疲れ様でした。終了します")
                break
            }
            else -> println("無効なコマンドです。数字か日本語で選んでください")
        }
    }
}

fun addTodo(todos: MutableList<String>){
    print("追加するタスクを入力してください： ")
    val task = readLine()?.trim()
    if(task.isNullOrEmpty()){
        println("エラー：空のタスクは追加できません")
    }else{
        todos.add(task)
        println("タスクを追加しました： \"$task\"（合計 ${todos.size} 件）")
    }
}

fun listTodos(todos: MutableList<String>){
    if(todos.isEmpty()){
        println("タスクがありません")
        return
    }
    println("===タスク一覧===")
    todos.forEachIndexed{ index, task ->
        println("${index + 1}: $task")
    }
}

fun removeTodo(todos: MutableList<String>){
    if(todos.isEmpty()){
        println("削除するタスクがありません")
        return
    }
    listTodos(todos)
    print("削除したい番号を選んでください： ")
    val num = readLine()?.trim()?.toIntOrNull()
    if(num == null || num < 1 || num > todos.size){
        println("無効な番号です。正しい番号を入力してください")
        }else{
            val removed = todos.removeAt(num - 1)
        println("削除しました： \"$removed\"（残り ${todos.size}件)")
    }
}