''''''
from cxxheaderparser.simple import parse_file
from cxxheaderparser.types import Pointer
from cxxheaderparser.types import Type
import string

parsed_data = parse_file('./ir_builder.h')

method_template = string.Template("""
pub fn ${func_name}(&mut self,${params}) -> ${return_type} {
    unsafe {
        ${return_type}::new(self.pin().${func_name}(${params_c}))
    }
}
""")

def ez_gen():
    passed = ['ReturnStmt', 'UnaryOpStmt', 'BinaryOpStmt', 'AtomicOpStmt', 'TernaryOpStmt']
    for mth in parsed_data.namespace.namespaces['taichi'].namespaces['lang'].classes[0].methods:
        if not (isinstance(mth.return_type, Pointer) and mth.return_type.ptr_to.typename.segments[0].name in passed):
            continue
        values = {}
        values['func_name'] = mth.name.segments[0].name
        params = []
        params_c  = []
        for param in mth.parameters:
            # print(param.type)
            if isinstance(param.type, Type):
                ty_name = param.type.typename.segments[0].name
                match ty_name:
                    case 'Stmt':
                        params.append(f" mut {param.name}: impl {ty_name}")
                        params_c.append(f" {param.name}.cast()")

                    case 'DataType':
                        params.append(f" {param.name}: types::DataType")
                        params_c.append(f" {param.name}.uni_ptr()")

            if isinstance(param.type, Pointer):
                ty_name = param.type.ptr_to.typename.segments[0].name
                match ty_name:
                    case 'Stmt':
                        params.append(f" mut {param.name}: impl {ty_name}")
                        params_c.append(f" {param.name}.cast()")

                    case 'DataType':
                        params.append(f" {param.name}: types::DataType")
                        params_c.append(f" {param.name}.uni_ptr()")

        values['params'] = ','.join(params)
        values['return_type'] = mth.return_type.ptr_to.typename.segments[0].name
        values['params_c'] = ','.join(params_c)
        print(method_template.substitute(values))

def kk_gen():
    passed = ['AdStackLoadTopAdjStmt', 'AdStackLoadTopStmt', 'MeshRelationAccessStmt', 'MeshRelationAccessStmt', 'MeshPatchIndexStmt']
    for mth in parsed_data.namespace.namespaces['taichi'].namespaces['lang'].classes[0].methods:
        if not mth.return_type.ptr_to.typename.segments[0].name in passed:
            continue
        mth_ret_type_name = ''
        match isinstance(mth.return_type, Type):
            case True:
                mth_ret_type_name = mth.return_type.typename.segments[0].name
            case False:
                mth_ret_type_name = mth.return_type.ptr_to.typename.segments[0].name
        values = {}
        
                
        
if __name__ == "__main__":
    kk_gen()