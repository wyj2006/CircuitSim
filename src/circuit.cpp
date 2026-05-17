#include <nvector/nvector_serial.h>
#include <sunlinsol/sunlinsol_spfgmr.h>
#include <sunmatrix/sunmatrix_sparse.h>
#include <sunnonlinsol/sunnonlinsol_newton.h>

#include "circuit.h"
#include "equation.h"

#define ZERO SUN_RCONST(0.0)

SUNContext sunctx;

int residual(sunrealtype t, N_Vector y0, N_Vector y1, N_Vector f,
             void *user_data)
{
    auto equations = *(vector<shared_ptr<Equation>> *)user_data;
    for (int i = 0; i < equations.size(); i++)
        NV_Ith_S(f, i) = equations[i]->evaluate(t, y0, y1);
    return 0;
}

void Circuit::simulate()
{
    // 分配所有变量
    size_t index = 0;
    for (auto node : nodes) node->voltage = make_shared<Variable>(index++);
    for (auto component : components)
        component->current = make_shared<Variable>(index++);

    vector<shared_ptr<Equation>> equations;

    // 创建初始值向量
    N_Vector y0_init = N_VNew_Serial(index, sunctx);
    N_Vector y1_init = N_VNew_Serial(index, sunctx);
    for (size_t i = 0; i < index; i++)
    {
        NV_Ith_S(y0_init, i) = ZERO;
        NV_Ith_S(y1_init, i) = ZERO;
    }

    // 创建矩阵对象
    SUNMatrix a = SUNSparseMatrix(index, index, index * index, CSC_MAT, sunctx);

    // 创建求解器对象
    N_Vector temp1 = N_VNew_Serial(index, sunctx);
    SUNLinearSolver ls = SUNLinSol_SPFGMR(temp1, SUN_PREC_RIGHT, 20, sunctx);

    N_Vector temp2 = N_VNew_Serial(index, sunctx);
    SUNNonlinearSolver nls = SUNNonlinSol_Newton(temp2, sunctx);

    // 创建 IDA 对象
    void *mem = IDACreate(sunctx);

    IDASetUserData(mem, &equations);

    // 初始化 IDA 求解器
    IDAInit(mem, residual, ZERO, y0_init, y1_init);

    // 指定积分容差
    N_Vector abstol = N_VNew_Serial(2, sunctx);
    NV_Ith_S(abstol, 0) = SUN_RCONST(1e-6);
    NV_Ith_S(abstol, 1) = SUN_RCONST(1e-9);
    IDASVtolerances(mem, SUN_RCONST(1e-3), abstol);

    // 附加线性求解器
    IDASetLinearSolver(mem, ls, nullptr);

    // 附加非线性求解器模块
    IDASetNonlinearSolver(mem, nls);

    // 随时间推进解
    sunrealtype tout = SUN_RCONST(0.001), tret;
    N_Vector y0_ret, y1_ret;
    for (int i = 0; i < 10; i++)
    {
        IDASolve(mem, tout, &tret, y0_ret, y1_ret, IDA_NORMAL);

        tout += SUN_RCONST(0.001);
    }
}