#!/usr/bin/python
# -*- coding: UTF-8 -*- 


import os
import shutil

def copy_file(file_name, folder_name):
    if os.path.exists(file_name):
        shutil.copy(file_name, folder_name)
        print('copy file '+ file_name + ' to ' + folder_name + ' succeed')
    else:
        print('**** error **** copy file '+file_name+' failed')

def copy_folder(folder_before, folder_after):           ## folder_after does not exist
    if os.path.exists(folder_before):
        shutil.copytree(folder_before, folder_after)
        print('copy dir ' + folder_before + ' to ' + folder_after + ' succeed')
    else:
        print('**** error **** copy folder '+folder_before+' failed')

def make_folder(folder_name):
    if os.path.exists(folder_name):
        shutil.rmtree(folder_name)
        print('delete ' + folder_name)
    os.makedirs(folder_name)
    print('make dir ' + folder_name)

def delete_folder(folder_name):
    if os.path.exists(folder_name):
        shutil.rmtree(folder_name)
        print('delete ' + folder_name)
    else:
        print(folder_name + ' does not exist, no need to delete')

def delete_file(file_name):
    if os.path.exists(file_name):
        os.remove(file_name)
        print('delete ' + file_name)
    else:
        print(file_name + ' does not exist, no need to delete')

def copy_files(srcDir, dstDir):                         ## dstDir exists
    for one_file in os.listdir(srcDir):
        one_file_path = os.path.join(srcDir, one_file)
        shutil.copy(one_file_path, dstDir)
    print('copy_files : from ' + srcDir + ' to ' + dstDir)

# ==================================================================== 

current_path = os.getcwd()
files = os.listdir(current_path)

def traverse(rootdir):
    l1 = os.listdir(rootdir)
    for i in range(0,len(l1)):
        path = os.path.join(rootdir, l1[i])
        if os.path.isdir(path):
            # delete target
            if l1[i] == 'target' or l1[i] == '.git':
                print(path)  
                delete_folder(path)          
            else:
                traverse(path)
        elif os.path.isfile(path):
            if l1[i] == '.gitignore' and path != '.\.gitignore':
                print(path)
                delete_file(path)

    return ;

traverse('.')