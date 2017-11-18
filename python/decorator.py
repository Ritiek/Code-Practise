def decorate(func):
    def wrapper():
        print('we are in wrapper()')
        func()
        print('did you just say whee?!')
    return wrapper


@decorate
def whee():
    print('whee!!')

whee()
