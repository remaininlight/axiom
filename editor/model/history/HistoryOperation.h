#pragma once

#include <QtCore/QDataStream>
#include <memory>

namespace AxiomModel {

    class Project;

    class HistoryOperation {
    public:
        enum class Type {
            ADD_NODE,
            DELETE_NODE
        };

        explicit HistoryOperation(bool needsRefresh, Type type);

        static std::unique_ptr<HistoryOperation> deserialize(Type type, QDataStream &stream, Project *project);

        Type type() const { return _type; }

        bool needsRefresh() const { return _needsRefresh; }

        virtual void forward() = 0;

        virtual void backward() = 0;

        virtual void serialize(QDataStream &stream) const = 0;

    private:
        bool _needsRefresh;
        Type _type;

    };

}
