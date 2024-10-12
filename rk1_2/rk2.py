"""Рубежный контроль представляет собой разработку тестов на языке Python.
1) Проведите рефакторинг текста программы рубежного контроля №1 таким образом,
 чтобы он был пригоден для модульного тестирования.
2) Для текста программы рубежного контроля №1 создайте модульные тесты с применением
 TDD - фреймворка (3 теста).
"""

class Chapter:
    def __init__(self, id, title, pages, book_id):
            self.id = id
            self.title = title
            self.pages = pages
            self.book_id = book_id

class Book:
      def __init__(self, id, name):
            self.id = id
            self.name = name

class ChapterBook:
      def __init__(self, book_id, chapter_id):
            self.book_id = book_id
            self.chapter_id = chapter_id

# данные
books = [
    Book(1, "Boeing 737 flight instruction(self-education)"),
    Book(2, "Международный центр торговли США. История"),
    Book(3, "Японские хокку самураев и камикадзе в картинках")
 ]

chapters = [
    Chapter(1,"Глава 1", 10, 1),
    Chapter(1,"Глава 1", 91, 1),
    Chapter(1,"Глава 1", 50, 2),
    Chapter(1,"Глава 1", 20, 3),
    Chapter(1,"Глава 1", 30, 3)
 ]

 # Запрос 1: Список всех книг, у которых название начинается с буквы "А", и список их глав
def query_books_starting_with_a():
    result = {}
    for book in books:
        if book.name.startswith('Я'):
            chapters_in_book = [ch.title for ch in chapters if ch.book_id == book.id]
            result[book.name] = chapters_in_book
    return result

# Запрос 2: Список книг с максимальной длиной глав
def query_books_with_max_chapter_length():
    result = {}
    for book in books:
        book_chapters = [ch for ch in chapters if ch.book_id == book.id]
        if book_chapters:
            max_chapter = max(book_chapters, key=lambda ch: ch.pages)
            result[book.name] = (max_chapter.title, max_chapter.pages)
    return dict(sorted(result.items(), key=lambda x: x[1][1], reverse=True))

# Запрос 3: Список всех связанных глав и книг, отсортированных по книгам
def query_all_related_chapters_and_books():
    result = {}
    for book in books:
        related_chapters = [ch.title for ch in chapters if ch.book_id == book.id]
        result[book.name] = related_chapters
    return result

# Выполнение запросов
print("Запрос 1:", query_books_starting_with_a())
print("Запрос 2:", query_books_with_max_chapter_length())
print("Запрос 3:", query_all_related_chapters_and_books())