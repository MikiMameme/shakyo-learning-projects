//サイコロシミュレーター

#include <iostream>
#include <random>
#include <vector>
#include <iomanip>

using namespace std;

//サイコロをふるための関数
int rollDice() {
    random_device rd;
    mt19937 gen(rd());
    uniform_int_distribution<> dist(1, 6);
    return dist(gen);
}

//統計情報を表示
void showStatus(const vector<int>& results) {
    vector<int> counts(7, 0);

    for (int num : results) {
        counts[num]++;
    }

    cout << "\n===統計情報===" << endl;
    for (int i = 1; i <= 6; ++i) {
        cout << i << "の目: " << counts[i] << "回 (";
        cout << fixed << setprecision(1);
        cout << (counts[i] * 100.0 / results.size()) << "%)" << endl;
    }
}

int main() {
    cout << "================================" << endl;
    cout << "　　サイコロシミュレーター" << endl;
    cout << "================================" << endl;

    vector<int> results;
    char choice;

    while (true) {
        cout << "\nEnterキーでサイコロを振ります(q:終了, s:統計)";
        cin.get(choice);

        if (choice == 'q' || choice == 'Q') {
            break;
        }

        if (choice == 's' || choice == 'S') {
            if (results.empty()) {
                cout << "まだサイコロを振っていません" << endl;
            }
            else {
                showStatus(results);
            }
            continue;
        }

        //サイコロを振る
        int result = rollDice();
        results.push_back(result);

        // アスキーアートで表示
        cout << "\n┌─────────┐" << endl;
        switch (result) {
        case 1:
            cout << "│         │" << endl;
            cout << "│    ●   │" << endl;
            cout << "│         │" << endl;
            break;
        case 2:
            cout << "│ ●      │" << endl;
            cout << "│         │" << endl;
            cout << "│       ●│" << endl;
            break;
        case 3:
            cout << "│ ●      │" << endl;
            cout << "│    ●   │" << endl;
            cout << "│       ●│" << endl;
            break;
        case 4:
            cout << "│ ●    ●│" << endl;
            cout << "│         │" << endl;
            cout << "│ ●    ●│" << endl;
            break;
        case 5:
            cout << "│ ●    ●│" << endl;
            cout << "│    ●   │" << endl;
            cout << "│ ●    ●│" << endl;
            break;
        case 6:
            cout << "│ ●    ●│" << endl;
            cout << "│ ●    ●│" << endl;
            cout << "│ ●    ●│" << endl;
            break;
        }
        cout << "└─────────┘" << endl;
        cout << "出目: " << result << " (合計 " << results.size() << "回)" << endl;
    }

    //最終統計
    if (!results.empty()) {
        cout << "\n================================" << endl;
        cout << " 最終結果" << endl;
        cout << "=================================" << endl;
        cout << "合計試行回数: " << results.size() << "回" << endl;
        showStatus(results);
    }

    cout << "\n終了します" << endl;
    return 0;
}