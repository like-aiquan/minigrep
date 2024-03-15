像grep命令一样查询文件夹下包含给定字符串的文件

|     参数      |           作用            |
| :-----------: | :-----------------------: |
|     path      |    指定的文件或文件夹     |
|      str      |       查询的字符串        |
| --ignore-case | 是否忽略大小写,默认不忽略 |
|      --s      |     指定文件后缀查询      |


```sh
minigrep [path] [str] [--ignore-case,--s=.rb/.rs]
```
从./assert/a.txt文件中查找 'How public, like a frog' 字符串就像这样
```sh
minigrep ./assert/a.txt 'How public, like a frog'
```

忽略大小写可以这样
```sh
minigrep ./assert/a.txt 'how Public, Like a Frog' --ignore-case
```

仅在后缀为.txt的文件中查找
```sh
minigrep ./assert/a.txt 'how Public, Like a Frog' --ignore-case --s=.txt
```

#### 简单的查询,支持的功能不多.参照 [构建一个简单命令行程序](https://course.rs/basic-practice/intro.html).
