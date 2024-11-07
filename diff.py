import difflib
import re

def file_diff(file1_path, file2_path):
    # 读取文件
    with open(file1_path, 'r') as file1, open(file2_path, 'r') as file2:
        file1_lines = file1.readlines()
        file2_lines = file2.readlines()
    
    # 获取两个文件的差异
    differ = difflib.Differ()
    diff = list(differ.compare(file1_lines, file2_lines))

    # 正则表达式，用于匹配大写字母开头，后跟冒号和空格
    pattern = re.compile(r'^[A-Z]{2,}: ')

    result = ""
    # 打印差异行和匹配特定模式的行
    for line in diff:
        if line.startswith('- '):  # 只关注有添加或删除的行
            # print(line.strip())            # 打印有差异的行
            if pattern.match(line.strip()[2:]):  # 检查是否匹配正则表达式
                # print("GNU:\t", line.strip()[2:])
                result += "GNU:" + line.strip()[2:] + "\n"
        elif line.startswith('+ '):
            # print(line.strip())            # 打印有差异的行
            if pattern.match(line.strip()[2:]):
                # print("Rust:\t", line.strip()[2:], "\n")
                result += "Rust:" + line.strip()[2:] + "\n\n"
    if result != "":
        return result
    else:
        return "No difference found.\n"

if __name__ == "__main__":
    result = "Functional Test Result: \n\n----------ld----------\n"
    file1_path = '/home/user/FunctionTest/binutils/ld/ld.sum'  # 第一个文件的路径
    file2_path = '/home/user/FunctionTest/rust-binutils/ld/ld.sum'  # 第二个文件的路径
    # file_diff(file1_path, file2_path)
    result += file_diff(file1_path, file2_path)

    result += "----------gas----------\n"
    file1_path = '/home/user/FunctionTest/binutils/gas/testsuite/gas.sum'  # 第一个文件的路径
    file2_path = '/home/user/FunctionTest/rust-binutils/gas/testsuite/gas.sum'  # 第二个文件的路径
    result += file_diff(file1_path, file2_path)

    result += "----------binutils----------\n"
    file1_path = '/home/user/FunctionTest/binutils/binutils/binutils.sum'  # 第一个文件的路径
    file2_path = '/home/user/FunctionTest/rust-binutils/binutils/binutils.sum'  # 第二个文件的路径
    result += file_diff(file1_path, file2_path)

    print(result)
    

