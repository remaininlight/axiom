#include "BeatsConverter.h"

#include "../MaximContext.h"
#include "../ComposableModuleClassMethod.h"

using namespace MaximCodegen;

BeatsConverter::BeatsConverter(MaximCodegen::MaximContext *ctx, llvm::Module *module)
    : Converter(ctx, module, MaximCommon::FormType::BEATS) {
    using namespace std::placeholders;
    converters.emplace(MaximCommon::FormType::CONTROL, std::bind(&BeatsConverter::fromControl, this, _1, _2));
    converters.emplace(MaximCommon::FormType::FREQUENCY, std::bind(&BeatsConverter::fromFrequency, this, _1, _2));
    converters.emplace(MaximCommon::FormType::SAMPLES, std::bind(&BeatsConverter::fromSamples, this, _1, _2));
    converters.emplace(MaximCommon::FormType::SECONDS, std::bind(&BeatsConverter::fromSeconds, this, _1, _2));
}

std::unique_ptr<BeatsConverter> BeatsConverter::create(MaximCodegen::MaximContext *ctx, llvm::Module *module) {
    return std::make_unique<BeatsConverter>(ctx, module);
}

llvm::Value* BeatsConverter::fromControl(MaximCodegen::ComposableModuleClassMethod *method, llvm::Value *val) {
    auto &b = method->builder();
    return b.CreateFMul(val, ctx()->constFloatVec(8));
}

llvm::Value* BeatsConverter::fromFrequency(MaximCodegen::ComposableModuleClassMethod *method, llvm::Value *val) {
    auto &b = method->builder();
    return b.CreateFDiv(
        b.CreateLoad(ctx()->beatsPerSecondPtr(*method->moduleClass()->module())),
        b.CreateFMul(ctx()->constFloatVec(60), val)
    );
}

llvm::Value* BeatsConverter::fromSamples(MaximCodegen::ComposableModuleClassMethod *method, llvm::Value *val) {
    auto &b = method->builder();
    return b.CreateFDiv(
        b.CreateFMul(val, b.CreateLoad(ctx()->beatsPerSecondPtr(*method->moduleClass()->module()))),
        b.CreateFMul(ctx()->constFloatVec(ctx()->sampleRate), ctx()->constFloatVec(60))
    );
}

llvm::Value* BeatsConverter::fromSeconds(MaximCodegen::ComposableModuleClassMethod *method, llvm::Value *val) {
    auto &b = method->builder();
    return b.CreateFMul(
        val,
        b.CreateFDiv(b.CreateLoad(ctx()->beatsPerSecondPtr(*method->moduleClass()->module())), ctx()->constFloatVec(60))
    );
}
