class Foo:
    def __init__(self):
        self.first_printed = False
        self.second_printed = False


    def first(self, printFirst: 'Callable[[], None]') -> None:
        
        # printFirst() outputs "first". Do not change or remove this line.
        printFirst()
        self.first_printed = True

    def second(self, printSecond: 'Callable[[], None]') -> None:
        while not self.first_printed:
            continue
        # printSecond() outputs "second". Do not change or remove this line.
        printSecond()
        self.second_printed = True


    def third(self, printThird: 'Callable[[], None]') -> None:
        while not self.second_printed:
            continue
        # printThird() outputs "third". Do not change or remove this line.
        printThird()