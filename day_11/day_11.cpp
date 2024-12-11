#include <iostream>
#include <list>
#include <vector>
#include <charconv>

void blink(std::list<int64_t> &stones)
{
    auto it = stones.begin();
    while (it != stones.end())
    {
        if (*it == 0)
        {
            *it = 1;
            it++;
            continue;
        }

        auto digits = std::to_string(*it);
        if (digits.size() % 2 == 0)
        {
            int64_t left, right;
            std::from_chars(digits.c_str(), digits.c_str() + digits.size() / 2, left);
            std::from_chars(digits.c_str() + digits.size() / 2, digits.c_str() + digits.size(), right);
            stones.insert(it, left);
            stones.insert(it, right);

            it = stones.erase(it);
            continue;
        }

        *it *= 2024;
        it++;
    }
}

int main()
{
    std::vector<int64_t> input;

    int64_t v;
    for (;;)
    {
        std::cin >> v;
        if (std::cin)
        {
            input.push_back(v);
        }
        else
        {
            break;
        }
    }

    std::list<int64_t> stones;
    for (const auto &i : input)
    {
        stones.push_back(i);
    }

    for (int i = 0; i < 25; ++i) {
        blink(stones);
    }

    std::cout << stones.size() << std::endl;
}
